use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Conf {
    pub cpn: String,
    pub asno: String,
    pub tag: String,
}

impl Conf {
    pub fn new(args: Vec<&str>) -> Result<Conf, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let tag = if args.len() > 2 {
            args[2].to_owned()
        } else {
            "VPN".to_owned()
        };
        let cpn = args[0].to_owned();
        let asno = args[1].to_owned();

        Ok(Conf { cpn, asno, tag })
    }
}

pub fn read_conf(file: &str) -> Result<Vec<Conf>, Box<dyn Error>> {
    let mut result = Vec::new();
    let contents = fs::read_to_string(format!("{}.txt", file))?;
    for line in contents.lines() {
        if line.contains(" ") {
            if let Ok(conf) = Conf::new(line.split(" ").collect()) {
                result.push(conf);
            }
        }
    }
    Ok(result)
}

pub fn run(file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::new();
    let ass = read_conf(file)?;
    for as_ in ass {
        let ips = read_as_ips(&as_);
        if let Ok(list) = ips {
            let mut first = true;
            for tmp in list {
                let sip = if first {
                    first = false;
                    format!(
                        "add list={} comment={}.{} address={}",
                        as_.tag, as_.cpn, as_.asno, tmp
                    )
                } else {
                    format!("add list={} address={}", as_.tag, tmp)
                };
                result.push(sip);
                //println!("{}", sip);
            }
        }
        // println!("{:?}", as_);
    }
    Ok(result)
}

pub fn read_as_ips(conf: &Conf) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(format!("{}.txt", conf.asno))?;
    Ok(contents
        .trim()
        .split(" ")
        .filter(|ip| ip.len() >= 7)
        .map(|ip| ip.trim().to_owned())
        .collect())
}
