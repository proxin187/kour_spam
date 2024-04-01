use reqwest::blocking::Client;
use reqwest::header;


const MESSAGE: &str = "<sprite=0><color=white><color=#7300FFFF>[DANK] </color>kashy</color>: <color=#e8e8e8>Im feeling quite skibidi";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    loop {
        let resp = client.post("https://kour.io/api/message")
            .body(format!(
                    "{{ \"message\": \"{}\" }}", MESSAGE))
            .header(header::COOKIE, "StatsSend=true")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
            .header(header::ORIGIN, "https://kour.io")
            .send()?
            .text()?;

        if resp.contains("success") {
            println!("[+] message sent");
        }
    }
}

