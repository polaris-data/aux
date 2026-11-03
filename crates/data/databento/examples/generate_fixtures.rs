//! Regenerate synthetic protocol fixtures using the pinned official DBN codec.
//! Does not contact Databento. Run from the workspace root.
use aux_rs_databento::dbn::{
    self, RecordMut,
    encode::{EncodeRecord, EncodeRecordRef},
};
use futures_util::io::AsyncWriteExt;
use std::path::Path;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let metadata = dbn::Metadata::builder()
        .dataset("GLBX.MDP3")
        .schema(None)
        .start(1704067200000000000)
        .stype_in(None)
        .stype_out(dbn::SType::InstrumentId)
        .build();
    let mut records: Vec<(&str, dbn::RecordBuf)> = vec![
        ("mbo", dbn::MboMsg::default().into()),
        ("trades", dbn::TradeMsg::default().into()),
        ("mbp-1", dbn::Mbp1Msg::default().into()),
        ("mbp-10", dbn::Mbp10Msg::default().into()),
        ("tbbo", dbn::Mbp1Msg::default().into()),
        ("definition", dbn::InstrumentDefMsg::default().into()),
        ("statistics", dbn::StatMsg::default().into()),
        ("status", dbn::StatusMsg::default().into()),
        ("imbalance", dbn::ImbalanceMsg::default().into()),
        (
            "cmbp-1",
            dbn::Cmbp1Msg::default_for_schema(dbn::Schema::Cmbp1).into(),
        ),
        (
            "tcbbo",
            dbn::Cmbp1Msg::default_for_schema(dbn::Schema::Tcbbo).into(),
        ),
        ("error", dbn::ErrorMsg::default().into()),
        ("system", dbn::SystemMsg::default().into()),
        ("symbol_mapping", dbn::SymbolMappingMsg::default().into()),
    ];
    for (name, rtype) in [
        ("ohlcv-1s", dbn::rtype::OHLCV_1S),
        ("ohlcv-1m", dbn::rtype::OHLCV_1M),
        ("ohlcv-1h", dbn::rtype::OHLCV_1H),
        ("ohlcv-1d", dbn::rtype::OHLCV_1D),
        ("ohlcv-eod", dbn::rtype::OHLCV_EOD),
    ] {
        let mut record = dbn::OhlcvMsg::default_for_schema(dbn::Schema::Ohlcv1S);
        record.hd.rtype = rtype;
        records.push((name, record.into()));
    }
    for (name, rtype) in [
        ("bbo-1s", dbn::rtype::BBO_1S),
        ("bbo-1m", dbn::rtype::BBO_1M),
    ] {
        let mut record = dbn::BboMsg::default_for_schema(dbn::Schema::Bbo1S);
        record.hd.rtype = rtype;
        records.push((name, record.into()));
    }
    for (name, rtype) in [
        ("cbbo-1s", dbn::rtype::CBBO_1S),
        ("cbbo-1m", dbn::rtype::CBBO_1M),
    ] {
        let mut record = dbn::CbboMsg::default_for_schema(dbn::Schema::Cbbo1S);
        record.hd.rtype = rtype;
        records.push((name, record.into()));
    }
    let mut bytes = Vec::new();
    let mut encoder = dbn::encode::dbn::Encoder::new(&mut bytes, &metadata)?;
    let mut expected = Vec::new();
    for (index, (name, record)) in records.iter_mut().enumerate() {
        record.header_mut().instrument_id = index as u32 + 1;
        record.header_mut().ts_event = 1704067200000000123;
        if let Some(trade) = record.get_mut::<dbn::TradeMsg>() {
            trade.price = 123456789123;
            trade.size = 3;
            trade.sequence = u32::MAX;
        }
        encoder.encode_record_ref(record.as_rec_ref())?;
        let mut json = Vec::new();
        dbn::encode::json::Encoder::new(&mut json, false, false, false)
            .encode_record_ref(record.as_rec_ref())?;
        expected.push(serde_json::json!({"schema":name,"record":serde_json::from_slice::<serde_json::Value>(&json)?}));
    }
    std::fs::write(root.join("schemas.dbn"), &bytes)?;
    std::fs::write(
        root.join("schemas.json"),
        serde_json::to_string_pretty(&expected)? + "\n",
    )?;
    let mut compressed = Vec::new();
    for part in bytes.chunks(bytes.len().div_ceil(2)) {
        let mut compressor = async_compression::futures::write::ZstdEncoder::new(Vec::new());
        compressor.write_all(part).await?;
        compressor.close().await?;
        compressed.extend(compressor.into_inner());
    }
    std::fs::write(root.join("schemas.dbn.zst"), compressed)?;
    let metadata = dbn::Metadata {
        ts_out: true,
        ..metadata
    };
    let mut bytes = Vec::new();
    let mut encoder = dbn::encode::dbn::Encoder::new(&mut bytes, &metadata)?;
    let record = dbn::WithTsOut::new(dbn::TradeMsg::default(), 1704067200000000999);
    encoder.encode_record(&record)?;
    std::fs::write(root.join("ts_out.dbn"), bytes)?;
    Ok(())
}
