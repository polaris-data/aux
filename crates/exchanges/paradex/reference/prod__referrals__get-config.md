> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# Get Config

GET https://api.prod.paradex.trade/v1/referrals/config

Reference: https://docs.paradex.trade/api/prod/referrals/get-config

## Examples

**SDK Code**

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('https://api.prod.paradex.trade/v1/referrals/config', headers = headers)

print(r.json())

```

```javascript
const headers = {
  'Accept':'application/json'
};

fetch('https://api.prod.paradex.trade/v1/referrals/config',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```go
package main
import (
      "bytes"
      "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"}
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "https://api.prod.paradex.trade/v1/referrals/config", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

```ruby
require 'uri'
require 'net/http'

url = URI("https://api.prod.paradex.trade/v1/referrals/config")

http = Net::HTTP.new(url.host, url.port)
http.use_ssl = true

request = Net::HTTP::Get.new(url)

response = http.request(request)
puts response.read_body
```

```java
import com.mashape.unirest.http.HttpResponse;
import com.mashape.unirest.http.Unirest;

HttpResponse<String> response = Unirest.get("https://api.prod.paradex.trade/v1/referrals/config")
  .asString();
```

```php
<?php
require_once('vendor/autoload.php');

$client = new \GuzzleHttp\Client();

$response = $client->request('GET', 'https://api.prod.paradex.trade/v1/referrals/config');

echo $response->getBody();
```

```csharp
using RestSharp;

var client = new RestClient("https://api.prod.paradex.trade/v1/referrals/config");
var request = new RestRequest(Method.GET);
IRestResponse response = client.Execute(request);
```

```swift
import Foundation

let request = NSMutableURLRequest(url: NSURL(string: "https://api.prod.paradex.trade/v1/referrals/config")! as URL,
                                        cachePolicy: .useProtocolCachePolicy,
                                    timeoutInterval: 10.0)
request.httpMethod = "GET"

let session = URLSession.shared
let dataTask = session.dataTask(with: request as URLRequest, completionHandler: { (data, response, error) -> Void in
  if (error != nil) {
    print(error as Any)
  } else {
    let httpResponse = response as? HTTPURLResponse
    print(httpResponse)
  }
})

dataTask.resume()
```