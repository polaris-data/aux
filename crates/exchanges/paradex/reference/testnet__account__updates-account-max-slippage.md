> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# Updates Account Max Slippage

POST https://api.testnet.paradex.trade/v1/account/profile/max_slippage

Reference: https://docs.paradex.trade/api/testnet/account/updates-account-max-slippage

## Examples

**SDK Code**

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json',
  'Authorization': 'Bearer {JWT}'
}

r = requests.post('https://api.testnet.paradex.trade/v1/account/profile/max_slippage', headers = headers)

print(r.json())

```

```javascript
const inputBody = {
  "max_slippage": "string"
};
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json',
  'Authorization':'Bearer {JWT}'
};

fetch('https://api.testnet.paradex.trade/v1/account/profile/max_slippage',
{
  method: 'POST',
  body: inputBody,
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
        "Content-Type": []string{"application/json"},
        "Accept": []string{"application/json"},
        "Authorization": []string{"Bearer {JWT}"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("POST", "https://api.testnet.paradex.trade/v1/account/profile/max_slippage", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

```ruby
require 'uri'
require 'net/http'

url = URI("https://api.testnet.paradex.trade/v1/account/profile/max_slippage")

http = Net::HTTP.new(url.host, url.port)
http.use_ssl = true

request = Net::HTTP::Post.new(url)

response = http.request(request)
puts response.read_body
```

```java
import com.mashape.unirest.http.HttpResponse;
import com.mashape.unirest.http.Unirest;

HttpResponse<String> response = Unirest.post("https://api.testnet.paradex.trade/v1/account/profile/max_slippage")
  .asString();
```

```php
<?php
require_once('vendor/autoload.php');

$client = new \GuzzleHttp\Client();

$response = $client->request('POST', 'https://api.testnet.paradex.trade/v1/account/profile/max_slippage');

echo $response->getBody();
```

```csharp
using RestSharp;

var client = new RestClient("https://api.testnet.paradex.trade/v1/account/profile/max_slippage");
var request = new RestRequest(Method.POST);
IRestResponse response = client.Execute(request);
```

```swift
import Foundation

let request = NSMutableURLRequest(url: NSURL(string: "https://api.testnet.paradex.trade/v1/account/profile/max_slippage")! as URL,
                                        cachePolicy: .useProtocolCachePolicy,
                                    timeoutInterval: 10.0)
request.httpMethod = "POST"

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