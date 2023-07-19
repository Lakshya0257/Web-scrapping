use error_chain::error_chain;
use reqwest;
use select::document::Document;
use select::predicate::{Name, Attr};
error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        Io(std::io::Error);
    }
}

#[tokio::main]
// async fn main() -> Result<()> {
//     let res = reqwest::get("https://github.com/YashRaj9211?tab=repositories")
//         .await?
//         .text()
//         .await?;

//     Document::from(res.as_str())
//         .find(Name("img"))
//         .filter_map(|n| n.attr("src"))
//         .for_each(|x| println!("{}", x));

//     Ok(())
// }

async fn main() -> Result<()> {
    let url = "https://www.flipkart.com/puma-hustle-v2-running-shoes-men/p/itm95cb6c25022cd?pid=SHOG5HF2HPJ6ZAHA&lid=LSTSHOG5HF2HPJ6ZAHAMIOM6L&marketplace=FLIPKART&store=osp%2Fcil&srno=b_1_7&otracker=browse&fm=organic&iid=f5e67a29-1733-4463-b350-10a40e8f5495.SHOG5HF2HPJ6ZAHA.SEARCH&ppt=browse&ppn=browse&ssid=mfcso8tue80000001689661447973";

    let res = reqwest::get(url).await?.text().await?;

    let document = Document::from(res.as_str());

    // Extracting prices using CSS selectors
    let prices = document
        .find(Attr("class", "_30jeq3 _16Jk6d"))
        .filter_map(|n| Some(n.text()))
        .collect::<Vec<_>>();

    // Print the extracted prices
    for price in prices {
        println!("{}", price);
    }

    Ok(())
}
