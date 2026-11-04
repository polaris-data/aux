use aux_rs_core::{ByteStream, TransportError};
use aux_rs_data::{BytesBody, DecodedBody, Error, Lines};
struct Fragments(tokio::sync::mpsc::Receiver<Vec<u8>>);
impl ByteStream for Fragments {
    async fn next(&mut self) -> Option<Result<Vec<u8>, TransportError>> {
        self.0.recv().await.map(Ok)
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        self.0.close();
        Ok(())
    }
}
#[tokio::test]
async fn cancelled_partial_line_retains_input() {
    let (tx, rx) = tokio::sync::mpsc::channel(4);
    tx.send(b"abc".to_vec()).await.unwrap();
    let mut lines = Lines::new(DecodedBody::new(Fragments(rx), "").unwrap());
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(10), lines.next())
            .await
            .is_err()
    );
    tx.send(b"def\n\nlast".to_vec()).await.unwrap();
    drop(tx);
    assert_eq!(lines.next().await.unwrap().unwrap(), b"abcdef");
    assert!(lines.next().await.unwrap().unwrap().is_empty());
    assert_eq!(lines.next().await.unwrap().unwrap(), b"last");
    assert!(lines.next().await.is_none());
}
#[tokio::test]
async fn gzip_members_and_corruption() {
    let data = include_bytes!("../../tardis/tests/fixtures/feed.ndjson.gz");
    let output = DecodedBody::new(BytesBody(Some(data.to_vec())), "gzip")
        .unwrap()
        .collect(1024 * 1024)
        .await
        .unwrap();
    assert!(output.starts_with(b"2024-01-01"));
    assert!(output.ends_with(b"\n\n"));
    let mut bad = data.to_vec();
    bad.truncate(bad.len() - 5);
    assert!(
        DecodedBody::new(BytesBody(Some(bad)), "gzip")
            .unwrap()
            .collect(1024 * 1024)
            .await
            .is_err()
    );
}
#[tokio::test]
async fn bounded_collect_and_line_limit() {
    assert!(
        DecodedBody::new(BytesBody(Some(vec![0; 100])), "")
            .unwrap()
            .collect(99)
            .await
            .is_err()
    );
    let mut lines = Lines::new(
        DecodedBody::new(BytesBody(Some(vec![b'x'; 16 * 1024 * 1024 + 1])), "").unwrap(),
    );
    assert!(matches!(lines.next().await, Some(Err(Error::Decode(_)))));
    assert!(lines.next().await.is_none());
}
#[cfg(feature = "files")]
#[tokio::test]
async fn failed_download_does_not_publish_or_leave_partial_files() {
    use aux_rs_core::{RequestOutcome, TransportErrorKind};
    struct Broken(bool);
    impl ByteStream for Broken {
        async fn next(&mut self) -> Option<Result<Vec<u8>, TransportError>> {
            if !self.0 {
                self.0 = true;
                Some(Ok(vec![1; 128]))
            } else {
                Some(Err(TransportError::new(
                    TransportErrorKind::Disconnected,
                    RequestOutcome::Unknown,
                    "fixture disconnected",
                )))
            }
        }
        async fn close(&mut self) -> Result<(), TransportError> {
            Ok(())
        }
    }
    let dir = std::env::temp_dir().join(format!("aux-download-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("data.gz");
    assert!(
        aux_rs_data::download_to(DecodedBody::new(Broken(false), "").unwrap(), &path)
            .await
            .is_err()
    );
    assert!(!path.exists());
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
    aux_rs_data::download_to(
        DecodedBody::new(BytesBody(Some(vec![2, 3])), "").unwrap(),
        &path,
    )
    .await
    .unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), vec![2, 3]);
    assert!(
        aux_rs_data::download_to(
            DecodedBody::new(BytesBody(Some(vec![9])), "").unwrap(),
            &path
        )
        .await
        .is_err()
    );
    assert_eq!(std::fs::read(&path).unwrap(), vec![2, 3]);
    std::fs::remove_dir_all(dir).unwrap();
}
