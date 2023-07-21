use std::{fmt, path::PathBuf, str::FromStr, time::Duration};

use clap::{Args, Parser};
use color_eyre::owo_colors::OwoColorize;
use futures::future::join_all;
use reqwest::{Client, Proxy, Url};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Path to the file containing the proxies
    #[arg(short, long)]
    input: PathBuf,
    #[command(flatten)]
    output: Output,
    /// Address to send the request to
    #[arg(short, long, default_value = "http://www.google.com")]
    ping_addr: Url,
    /// Timeout for the request in seconds
    #[arg(short, long, default_value = "10")]
    timeout: u64,
    /// Print more information
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Args)]
#[group(multiple = false)]
struct Output {
    /// Path to the file to write the proxies to
    #[arg(short, long, default_value = "./output.txt")]
    output_file: Option<PathBuf>,
    /// Print the proxies to stdout
    #[arg(long)]
    stdout: bool,
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    let mut args = Arguments::parse();
    if args.output.stdout {
        args.output.output_file = None;
    }
    eprintln!("{:#?}", args);
    let Arguments {
        input,
        output: Output {
            output_file,
            stdout,
        },
        timeout,
        ping_addr,
        verbose,
    } = args;
    let timeout = Duration::from_secs(timeout);
    let input_file = fs::read_to_string(input).await?;

    let proxies = input_file
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(Url::from_str)
        .try_fold(Vec::new(), |mut acc, proxy| {
            acc.push(proxy?);
            Result::<Vec<Url>, <Url as FromStr>::Err>::Ok(acc)
        })?;

    eprintln!("Proxies: {}", DisplaySlice(&proxies));

    let requests = proxies
        .into_iter()
        .filter_map(|proxy_addr| {
            Proxy::http(proxy_addr.clone())
                .map_err(|err| println!("{}", format!("Error creating proxy: {err}").red()))
                .ok()
                .map(|proxy| (proxy_addr, proxy))
        })
        .filter_map(|(proxy_addr, proxy)| {
            Client::builder()
                .proxy(proxy)
                .timeout(timeout)
                .build()
                .map_err(|err| println!("{}", format!("Error building client: {err}").red()))
                .ok()
                .map(|client| (proxy_addr, client))
        })
        .map({
            move |(proxy_addr, client)| {
                let ping_addr = ping_addr.clone();
                async move { client.get(ping_addr).send().await.map(|_| proxy_addr) }
            }
        })
        .collect::<Vec<_>>();

    let mut file = match output_file {
        Some(path) => Some(File::create(path).await?),
        None => None,
    };
    for result in join_all(requests).await {
        match result {
            Ok(proxy_addr) => {
                if stdout {
                    if verbose {
                        println!("{}", format!("Success: {}", proxy_addr).green());
                    } else {
                        println!("{}", proxy_addr.green());
                    }
                }
                if let Some(file) = &mut file {
                    file.write_all(format!("{}\n", proxy_addr).as_bytes())
                        .await?;
                }
            }
            Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
        }
    }

    Ok(())
}

struct DisplaySlice<'a, T>(&'a [T]);

impl<'a, T: fmt::Display> fmt::Display for DisplaySlice<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for item in iter {
                write!(f, ", {}", item)?;
            }
        }
        write!(f, "]")
    }
}
