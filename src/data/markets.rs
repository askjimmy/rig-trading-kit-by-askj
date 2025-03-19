// src/market_data.rs
use lazy_static::lazy_static;
use serde_json::json;
use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    pub static ref MARKET_DATA: String = vec![
        "Spot Market 0: \"USDC\"",
        "Spot Market 1: \"SOL\"",
        "Spot Market 2: \"mSOL\"",
        "Spot Market 3: \"wBTC\"",
        "Spot Market 4: \"wETH\"",
        "Spot Market 5: \"USDT\"",
        "Spot Market 6: \"jitoSOL\"",
        "Spot Market 7: \"PYTH\"",
        "Spot Market 8: \"bSOL\"",
        "Spot Market 9: \"JTO\"",
        "Spot Market 10: \"WIF\"",
        "Spot Market 11: \"JUP\"",
        "Spot Market 12: \"RENDER\"",
        "Spot Market 13: \"W\"",
        "Spot Market 14: \"TNSR\"",
        "Spot Market 15: \"DRIFT\"",
        "Spot Market 16: \"INF\"",
        "Spot Market 17: \"dSOL\"",
        "Spot Market 18: \"USDY\"",
        "Spot Market 19: \"JLP\"",
        "Spot Market 20: \"POPCAT\"",
        "Spot Market 21: \"CLOUD\"",
        "Spot Market 22: \"PYUSD\"",
        "Spot Market 23: \"USDe\"",
        "Spot Market 24: \"sUSDe\"",
        "Spot Market 25: \"BNSOL\"",
        "Spot Market 26: \"MOTHER\"",
        "Spot Market 27: \"cbBTC\"",
        "Spot Market 28: \"USDS\"",
        "Spot Market 29: \"META\"",
        "Spot Market 30: \"ME\"",
        "Spot Market 31: \"PENGU\"",
        "Spot Market 32: \"Bonk\"",
        "Spot Market 33: \"JLP-1\"",
        "Spot Market 34: \"USDC-1\"",
        "Spot Market 35: \"AI16Z\"",
        "Spot Market 36: \"TRUMP\"",
        "Spot Market 37: \"MELANIA\"",
        "Spot Market 38: \"AUSD\"",
        "Perp Market 0: \"SOL-PERP\"",
        "Perp Market 1: \"BTC-PERP\"",
        "Perp Market 2: \"ETH-PERP\"",
        "Perp Market 3: \"APT-PERP\"",
        "Perp Market 4: \"1MBONK-PERP\"",
        "Perp Market 5: \"POL-PERP\"",
        "Perp Market 6: \"ARB-PERP\"",
        "Perp Market 7: \"DOGE-PERP\"",
        "Perp Market 8: \"BNB-PERP\"",
        "Perp Market 9: \"SUI-PERP\"",
        "Perp Market 10: \"1MPEPE-PERP\"",
        "Perp Market 11: \"OP-PERP\"",
        "Perp Market 12: \"RENDER-PERP\"",
        "Perp Market 13: \"XRP-PERP\"",
        "Perp Market 14: \"HNT-PERP\"",
        "Perp Market 15: \"INJ-PERP\"",
        "Perp Market 16: \"LINK-PERP\"",
        "Perp Market 17: \"RLB-PERP\"",
        "Perp Market 18: \"PYTH-PERP\"",
        "Perp Market 19: \"TIA-PERP\"",
        "Perp Market 20: \"JTO-PERP\"",
        "Perp Market 21: \"SEI-PERP\"",
        "Perp Market 22: \"AVAX-PERP\"",
        "Perp Market 23: \"WIF-PERP\"",
        "Perp Market 24: \"JUP-PERP\"",
        "Perp Market 25: \"DYM-PERP\"",
        "Perp Market 26: \"TAO-PERP\"",
        "Perp Market 27: \"W-PERP\"",
        "Perp Market 28: \"KMNO-PERP\"",
        "Perp Market 29: \"TNSR-PERP\"",
        "Perp Market 30: \"DRIFT-PERP\"",
        "Perp Market 31: \"CLOUD-PERP\"",
        "Perp Market 32: \"IO-PERP\"",
        "Perp Market 33: \"ZEX-PERP\"",
        "Perp Market 34: \"POPCAT-PERP\"",
        "Perp Market 35: \"1KWEN-PERP\"",
        "Perp Market 42: \"TON-PERP\"",
        "Perp Market 44: \"MOTHER-PERP\"",
        "Perp Market 45: \"MOODENG-PERP\"",
        "Perp Market 47: \"DBR-PERP\"",
        "Perp Market 48: \"WLF-5B-1W-BET\"",
        "Perp Market 51: \"1KMEW-PERP\"",
        "Perp Market 52: \"MICHI-PERP\"",
        "Perp Market 53: \"GOAT-PERP\"",
        "Perp Market 54: \"FWOG-PERP\"",
        "Perp Market 55: \"PNUT-PERP\"",
        "Perp Market 56: \"RAY-PERP\"",
        "Perp Market 59: \"HYPE-PERP\"",
        "Perp Market 60: \"LTC-PERP\"",
        "Perp Market 61: \"ME-PERP\"",
        "Perp Market 62: \"PENGU-PERP\"",
        "Perp Market 63: \"AI16Z-PERP\"",
        "Perp Market 64: \"TRUMP-PERP\"",
        "Perp Market 65: \"MELANIA-PERP\"",
        "Perp Market 66: \"BERA-PERP\"",
        "Perp Market 67: \"NBAFINALS25-OKC-BET\"",
        "Perp Market 68: \"NBAFINALS25-BOS-BET\"",
        "Perp Market 69: \"KAITO-PERP\"",
        "Perp Market 70: \"IP-PERP\"",
    ].join("\n").to_string();
}


pub fn parse_market_data() -> serde_json::Value {
    let mut spot_markets = HashMap::new();
    let mut perp_markets = HashMap::new();
    
    let re = Regex::new(r#"^(Spot|Perp) Market (\d+): "([^"]+)"$"#).unwrap();

    for line in MARKET_DATA.lines() {
        if let Some(caps) = re.captures(line) {
            let market_type = &caps[1];
            let market_id = &caps[2];
            let market_name = &caps[3];

            if market_type == "Spot" {
                spot_markets.insert(market_id.to_string(), market_name.to_string());
            } else {
                perp_markets.insert(market_id.to_string(), market_name.to_string());
            }
        }
    }

    json!({
        "SPOT": spot_markets,
        "PERP": perp_markets
    })
}