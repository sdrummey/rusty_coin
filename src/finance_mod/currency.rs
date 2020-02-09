// SOURCE: https://www.xe.com/symbols.php

// NOTE: The primary type is the CurrCode as it's required by value::Value. With the CurrCode, you
// can then use the currency() method to obtain the Currency information for the respective code

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Currency {
    pub country: &'static str,
    pub curr_type: CurrType,
    pub code: CurrCode,
    pub symbol: &'static str,
}

impl Currency {
    pub fn new(country: &'static str, curr_type: CurrType, code: CurrCode) -> Currency {
        Currency{
            country,
            curr_type,
            code,
            symbol: code.currency().symbol
        }
    }

}

// https://stevedonovan.github.io/rust-gentle-intro/2-structs-enums-lifetimes.html#simple-enums
impl CurrCode {
    pub fn currency(&self) -> Currency {
        match self {
            CurrCode::ALL => Currency { country: "Albania", curr_type: CurrType::Lek, code: CurrCode::ALL, symbol: "Lek" },
            CurrCode::AFN => Currency { country: "Afghanistan", curr_type: CurrType::Afghani, code: CurrCode::AFN, symbol: "؋" },
            CurrCode::ARS => Currency { country: "Argentina", curr_type: CurrType::Peso, code: CurrCode::ARS, symbol: "$" },
            CurrCode::AWG => Currency { country: "Aruba", curr_type: CurrType::Guilder, code: CurrCode::AWG, symbol: "ƒ" },
            CurrCode::AUD => Currency { country: "Australia", curr_type: CurrType::Dollar, code: CurrCode::AUD, symbol: "$" },
            CurrCode::AZN => Currency { country: "Azerbaijan", curr_type: CurrType::Manat, code: CurrCode::AZN, symbol: "₼" },
            CurrCode::BSD => Currency { country: "Bahamas", curr_type: CurrType::Dollar, code: CurrCode::BSD, symbol: "$" },
            CurrCode::BBD => Currency { country: "Barbados", curr_type: CurrType::Dollar, code: CurrCode::BBD, symbol: "$" },
            CurrCode::BYN => Currency { country: "Belarus", curr_type: CurrType::Ruble, code: CurrCode::BYN, symbol: "Br" },
            CurrCode::BZD => Currency { country: "Belize", curr_type: CurrType::Dollar, code: CurrCode::BZD, symbol: "BZ$" },
            CurrCode::BMD => Currency { country: "Bermuda", curr_type: CurrType::Dollar, code: CurrCode::BMD, symbol: "$" },
            CurrCode::BOB => Currency { country: "Bolivia", curr_type: CurrType::Boliviano, code: CurrCode::BOB, symbol: "$b" },
            CurrCode::BAM => Currency { country: "Bosnia and Herzegovina", curr_type: CurrType::ConvertibleMark, code: CurrCode::BAM, symbol: "KM" },
            CurrCode::BWP => Currency { country: "Botswana", curr_type: CurrType::Pula, code: CurrCode::BWP, symbol: "P" },
            CurrCode::BGN => Currency { country: "Bulgaria", curr_type: CurrType::Lev, code: CurrCode::BGN, symbol: "лв" },
            CurrCode::BRL => Currency { country: "Brazil", curr_type: CurrType::Real, code: CurrCode::BRL, symbol: "R$" },
            CurrCode::BND => Currency { country: "Brunei Darussalam", curr_type: CurrType::Dollar, code: CurrCode::BND, symbol: "$" },
            CurrCode::KHR => Currency { country: "Cambodia", curr_type: CurrType::Riel, code: CurrCode::KHR, symbol: "៛" },
            CurrCode::CAD => Currency { country: "Canada", curr_type: CurrType::Dollar, code: CurrCode::CAD, symbol: "$" },
            CurrCode::KYD => Currency { country: "Cayman Islands", curr_type: CurrType::Dollar, code: CurrCode::KYD, symbol: "$" },
            CurrCode::CLP => Currency { country: "Chile", curr_type: CurrType::Peso, code: CurrCode::CLP, symbol: "$" },
            CurrCode::CNY => Currency { country: "China", curr_type: CurrType::Yuan, code: CurrCode::CNY, symbol: "¥" },
            CurrCode::COP => Currency { country: "Colombia", curr_type: CurrType::Peso, code: CurrCode::COP, symbol: "$" },
            CurrCode::CRC => Currency { country: "Costa Rica", curr_type: CurrType::Colon, code: CurrCode::CRC, symbol: "₡" },
            CurrCode::HRK => Currency { country: "Croatia", curr_type: CurrType::Kuna, code: CurrCode::HRK, symbol: "kn" },
            CurrCode::CUP => Currency { country: "Cuba", curr_type: CurrType::Peso, code: CurrCode::CUP, symbol: "₱" },
            CurrCode::CZK => Currency { country: "Czech Republic", curr_type: CurrType::Koruna, code: CurrCode::CZK, symbol: "Kč" },
            CurrCode::DKK => Currency { country: "Denmark", curr_type: CurrType::Krone, code: CurrCode::DKK, symbol: "kr" },
            CurrCode::DOP => Currency { country: "Dominican Republic", curr_type: CurrType::Peso, code: CurrCode::DOP, symbol: "RD$" },
            CurrCode::XCD => Currency { country: "East Caribbean", curr_type: CurrType::Dollar, code: CurrCode::XCD, symbol: "$" },
            CurrCode::EGP => Currency { country: "Egypt", curr_type: CurrType::Pound, code: CurrCode::EGP, symbol: "£" },
            CurrCode::SVC => Currency { country: "El Salvador", curr_type: CurrType::Colon, code: CurrCode::SVC, symbol: "$" },
            CurrCode::EUR => Currency { country: "Euro Member Countries", curr_type: CurrType::Euro, code: CurrCode::EUR, symbol: "€" },
            CurrCode::FKP => Currency { country: "Falkland Islands", curr_type: CurrType::Pound, code: CurrCode::FKP, symbol: "£" },
            CurrCode::FJD => Currency { country: "Fiji", curr_type: CurrType::Dollar, code: CurrCode::FJD, symbol: "$" },
            CurrCode::GHS => Currency { country: "Ghana", curr_type: CurrType::Cedi, code: CurrCode::GHS, symbol: "¢" },
            CurrCode::GIP => Currency { country: "Gibraltar", curr_type: CurrType::Pound, code: CurrCode::GIP, symbol: "£" },
            CurrCode::GTQ => Currency { country: "Guatemala", curr_type: CurrType::Quetzal, code: CurrCode::GTQ, symbol: "Q" },
            CurrCode::GGP => Currency { country: "Guernsey", curr_type: CurrType::Pound, code: CurrCode::GGP, symbol: "£" },
            CurrCode::GYD => Currency { country: "Guyana", curr_type: CurrType::Dollar, code: CurrCode::GYD, symbol: "$" },
            CurrCode::HNL => Currency { country: "Honduras", curr_type: CurrType::Lempira, code: CurrCode::HNL, symbol: "L" },
            CurrCode::HKD => Currency { country: "Hong Kong", curr_type: CurrType::Dollar, code: CurrCode::HKD, symbol: "$" },
            CurrCode::HUF => Currency { country: "Hungary", curr_type: CurrType::Forint, code: CurrCode::HUF, symbol: "Ft" },
            CurrCode::ISK => Currency { country: "Iceland", curr_type: CurrType::Krona, code: CurrCode::ISK, symbol: "kr" },
            CurrCode::INR => Currency { country: "India", curr_type: CurrType::Rupee, code: CurrCode::INR, symbol: "" },
            CurrCode::IDR => Currency { country: "Indonesia", curr_type: CurrType::Rupiah, code: CurrCode::IDR, symbol: "Rp" },
            CurrCode::IRR => Currency { country: "Iran", curr_type: CurrType::Rial, code: CurrCode::IRR, symbol: "﷼" },
            CurrCode::IMP => Currency { country: "Isle of Man", curr_type: CurrType::Pound, code: CurrCode::IMP, symbol: "£" },
            CurrCode::ILS => Currency { country: "Israel", curr_type: CurrType::Shekel, code: CurrCode::ILS, symbol: "₪" },
            CurrCode::JMD => Currency { country: "Jamaica", curr_type: CurrType::Dollar, code: CurrCode::JMD, symbol: "J$" },
            CurrCode::JPY => Currency { country: "Japan", curr_type: CurrType::Yen, code: CurrCode::JPY, symbol: "¥" },
            CurrCode::JEP => Currency { country: "Jersey", curr_type: CurrType::Pound, code: CurrCode::JEP, symbol: "£" },
            CurrCode::KZT => Currency { country: "Kazakhstan", curr_type: CurrType::Tenge, code: CurrCode::KZT, symbol: "лв" },
            CurrCode::KPW => Currency { country: "Korea (North)", curr_type: CurrType::Won, code: CurrCode::KPW, symbol: "₩" },
            CurrCode::KRW => Currency { country: "Korea (South)", curr_type: CurrType::Won, code: CurrCode::KRW, symbol: "₩" },
            CurrCode::KGS => Currency { country: "Kyrgyzstan", curr_type: CurrType::Som, code: CurrCode::KGS, symbol: "лв" },
            CurrCode::LAK => Currency { country: "Laos", curr_type: CurrType::Kip, code: CurrCode::LAK, symbol: "₭" },
            CurrCode::LBP => Currency { country: "Lebanon", curr_type: CurrType::Pound, code: CurrCode::LBP, symbol: "£" },
            CurrCode::LRD => Currency { country: "Liberia", curr_type: CurrType::Dollar, code: CurrCode::LRD, symbol: "$" },
            CurrCode::MKD => Currency { country: "Macedonia", curr_type: CurrType::Denar, code: CurrCode::MKD, symbol: "ден" },
            CurrCode::MYR => Currency { country: "Malaysia", curr_type: CurrType::Ringgit, code: CurrCode::MYR, symbol: "RM" },
            CurrCode::MUR => Currency { country: "Mauritius", curr_type: CurrType::Rupee, code: CurrCode::MUR, symbol: "₨" },
            CurrCode::MXN => Currency { country: "Mexico", curr_type: CurrType::Peso, code: CurrCode::MXN, symbol: "$" },
            CurrCode::MNT => Currency { country: "Mongolia", curr_type: CurrType::Tughrik, code: CurrCode::MNT, symbol: "₮" },
            CurrCode::MZN => Currency { country: "Mozambique", curr_type: CurrType::Metical, code: CurrCode::MZN, symbol: "MT" },
            CurrCode::NAD => Currency { country: "Namibia", curr_type: CurrType::Dollar, code: CurrCode::NAD, symbol: "$" },
            CurrCode::NPR => Currency { country: "Nepal", curr_type: CurrType::Rupee, code: CurrCode::NPR, symbol: "₨" },
            CurrCode::ANG => Currency { country: "Netherlands Antilles", curr_type: CurrType::Guilder, code: CurrCode::ANG, symbol: "ƒ" },
            CurrCode::NZD => Currency { country: "New Zealand", curr_type: CurrType::Dollar, code: CurrCode::NZD, symbol: "$" },
            CurrCode::NIO => Currency { country: "Nicaragua", curr_type: CurrType::Cordoba, code: CurrCode::NIO, symbol: "C$" },
            CurrCode::NGN => Currency { country: "Nigeria", curr_type: CurrType::Naira, code: CurrCode::NGN, symbol: "₦" },
            CurrCode::NOK => Currency { country: "Norway", curr_type: CurrType::Krone, code: CurrCode::NOK, symbol: "kr" },
            CurrCode::OMR => Currency { country: "Oman", curr_type: CurrType::Rial, code: CurrCode::OMR, symbol: "﷼" },
            CurrCode::PKR => Currency { country: "Pakistan", curr_type: CurrType::Rupee, code: CurrCode::PKR, symbol: "₨" },
            CurrCode::PAB => Currency { country: "Panama", curr_type: CurrType::Balboa, code: CurrCode::PAB, symbol: "B/." },
            CurrCode::PYG => Currency { country: "Paraguay", curr_type: CurrType::Guarani, code: CurrCode::PYG, symbol: "Gs" },
            CurrCode::PEN => Currency { country: "Peru", curr_type: CurrType::Sol, code: CurrCode::PEN, symbol: "S/." },
            CurrCode::PHP => Currency { country: "Philippines", curr_type: CurrType::Peso, code: CurrCode::PHP, symbol: "₱" },
            CurrCode::PLN => Currency { country: "Poland", curr_type: CurrType::Zloty, code: CurrCode::PLN, symbol: "zł" },
            CurrCode::QAR => Currency { country: "Qatar", curr_type: CurrType::Riyal, code: CurrCode::QAR, symbol: "﷼" },
            CurrCode::RON => Currency { country: "Romania", curr_type: CurrType::Leu, code: CurrCode::RON, symbol: "lei" },
            CurrCode::RUB => Currency { country: "Russia", curr_type: CurrType::Ruble, code: CurrCode::RUB, symbol: "₽" },
            CurrCode::SHP => Currency { country: "Saint Helena", curr_type: CurrType::Pound, code: CurrCode::SHP, symbol: "£" },
            CurrCode::SAR => Currency { country: "Saudi Arabia", curr_type: CurrType::Riyal, code: CurrCode::SAR, symbol: "﷼" },
            CurrCode::RSD => Currency { country: "Serbia", curr_type: CurrType::Dinar, code: CurrCode::RSD, symbol: "Дин." },
            CurrCode::SCR => Currency { country: "Seychelles", curr_type: CurrType::Rupee, code: CurrCode::SCR, symbol: "₨" },
            CurrCode::SGD => Currency { country: "Singapore", curr_type: CurrType::Dollar, code: CurrCode::SGD, symbol: "$" },
            CurrCode::SBD => Currency { country: "Solomon Islands", curr_type: CurrType::Dollar, code: CurrCode::SBD, symbol: "$" },
            CurrCode::SOS => Currency { country: "Somalia", curr_type: CurrType::Shilling, code: CurrCode::SOS, symbol: "S" },
            CurrCode::ZAR => Currency { country: "South Africa", curr_type: CurrType::Rand, code: CurrCode::ZAR, symbol: "R" },
            CurrCode::LKR => Currency { country: "Sri Lanka", curr_type: CurrType::Rupee, code: CurrCode::LKR, symbol: "₨" },
            CurrCode::SEK => Currency { country: "Sweden", curr_type: CurrType::Krona, code: CurrCode::SEK, symbol: "kr" },
            CurrCode::CHF => Currency { country: "Switzerland", curr_type: CurrType::Franc, code: CurrCode::CHF, symbol: "CHF" },
            CurrCode::SRD => Currency { country: "Suriname", curr_type: CurrType::Dollar, code: CurrCode::SRD, symbol: "$" },
            CurrCode::SYP => Currency { country: "Syria", curr_type: CurrType::Pound, code: CurrCode::SYP, symbol: "£" },
            CurrCode::TWD => Currency { country: "Taiwan New", curr_type: CurrType::Dollar, code: CurrCode::TWD, symbol: "NT$" },
            CurrCode::THB => Currency { country: "Thailand", curr_type: CurrType::Baht, code: CurrCode::THB, symbol: "฿" },
            CurrCode::TTD => Currency { country: "Trinidad and Tobago", curr_type: CurrType::Dollar, code: CurrCode::TTD, symbol: "TT$" },
            CurrCode::TRY => Currency { country: "Turkey", curr_type: CurrType::Lira, code: CurrCode::TRY, symbol: "₺" },
            CurrCode::TVD => Currency { country: "Tuvalu", curr_type: CurrType::Dollar, code: CurrCode::TVD, symbol: "$" },
            CurrCode::UAH => Currency { country: "Ukraine", curr_type: CurrType::Hryvnia, code: CurrCode::UAH, symbol: "₴" },
            CurrCode::GBP => Currency { country: "United Kingdom", curr_type: CurrType::Pound, code: CurrCode::GBP, symbol: "£" },
            CurrCode::USD => Currency { country: "United States", curr_type: CurrType::Dollar, code: CurrCode::USD, symbol: "$" },
            CurrCode::UYU => Currency { country: "Uruguay", curr_type: CurrType::Peso, code: CurrCode::UYU, symbol: "$U" },
            CurrCode::UZS => Currency { country: "Uzbekistan", curr_type: CurrType::Som, code: CurrCode::UZS, symbol: "лв" },
            CurrCode::VEF => Currency { country: "Venezuela", curr_type: CurrType::Bolivar, code: CurrCode::VEF, symbol: "Bs" },
            CurrCode::VND => Currency { country: "Viet Nam", curr_type: CurrType::Dong, code: CurrCode::VND, symbol: "₫" },
            CurrCode::YER => Currency { country: "Yemen", curr_type: CurrType::Rial, code: CurrCode::YER, symbol: "﷼" },
            CurrCode::ZWD => Currency { country: "Zimbabwe", curr_type: CurrType::Dollar, code: CurrCode::ZWD, symbol: "Z$" },

        }
    }
}


#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum CurrType {
    Lek,
    Afghani,
    Peso,
    Guilder,
    Dollar,
    Manat,
    Ruble,
    Boliviano,
    ConvertibleMark,
    Pula,
    Lev,
    Real,
    Riel,
    Yuan,
    Colon,
    Kuna,
    Koruna,
    Krone,
    Pound,
    Euro,
    Cedi,
    Quetzal,
    Lempira,
    Forint,
    Krona,
    Rupee,
    Rupiah,
    Rial,
    Shekel,
    Yen,
    Tenge,
    Won,
    Som,
    Kip,
    Denar,
    Ringgit,
    Tughrik,
    Metical,
    Cordoba,
    Naira,
    Balboa,
    Guarani,
    Sol,
    Zloty,
    Riyal,
    Leu,
    Dinar,
    Shilling,
    Rand,
    Franc,
    Baht,
    Lira,
    Hryvnia,
    Bolivar,
    Dong,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum CurrCode {
    ALL,
    AFN,
    ARS,
    AWG,
    AUD,
    AZN,
    BSD,
    BBD,
    BYN,
    BZD,
    BMD,
    BOB,
    BAM,
    BWP,
    BGN,
    BRL,
    BND,
    KHR,
    CAD,
    KYD,
    CLP,
    CNY,
    COP,
    CRC,
    HRK,
    CUP,
    CZK,
    DKK,
    DOP,
    XCD,
    EGP,
    SVC,
    EUR,
    FKP,
    FJD,
    GHS,
    GIP,
    GTQ,
    GGP,
    GYD,
    HNL,
    HKD,
    HUF,
    ISK,
    INR,
    IDR,
    IRR,
    IMP,
    ILS,
    JMD,
    JPY,
    JEP,
    KZT,
    KPW,
    KRW,
    KGS,
    LAK,
    LBP,
    LRD,
    MKD,
    MYR,
    MUR,
    MXN,
    MNT,
    MZN,
    NAD,
    NPR,
    ANG,
    NZD,
    NIO,
    NGN,
    NOK,
    OMR,
    PKR,
    PAB,
    PYG,
    PEN,
    PHP,
    PLN,
    QAR,
    RON,
    RUB,
    SHP,
    SAR,
    RSD,
    SCR,
    SGD,
    SBD,
    SOS,
    ZAR,
    LKR,
    SEK,
    CHF,
    SRD,
    SYP,
    TWD,
    THB,
    TTD,
    TRY,
    TVD,
    UAH,
    GBP,
    USD,
    UYU,
    UZS,
    VEF,
    VND,
    YER,
    ZWD,
}


//pub struct Country {
//    name: String,
//    currency: Currency
//}

//impl Country {
//
//    pub fn new() -> Country {
//        Country {
//
//        }
//    }
//
//}


//pub enum CountryName {
//    Albania,
//    Afghanistan,
//    Argentina,
//    Aruba,
//    Australia,
//    Azerbaijan,
//    Bahamas,
//    Barbados,
//    Belarus,
//    Belize,
//    Bermuda,
//    Bolivia,
//    BosniaHerzegovina,
//    Botswana,
//    Bulgaria,
//    Brazil,
//    BruneiDarussalam,
//    Cambodia,
//    Canada,
//    CaymanIslands,
//    Chile,
//    China,
//    Colombia,
//    CostaRica,
//Croatia,
//Cuba,
//CzechRepublic,
//Denmark,
//DominicanRepublic,
//EastCaribbean,
//Egypt,
//ElSalvador,
//Euro Member Countries,
//Falkland Islands,
//Fiji,
//Ghana,
//Gibraltar,
//Guatemala,
//Guernsey,
//Guyana,
//Honduras,
//Hong Kong,
//Hungary,
//Iceland,
//India,
//Indonesia,
//Iran,
//Isle of Man,
//Israel,
//Jamaica,
//Japan,
//Jersey,
//Kazakhstan,
//Korea (North),
//Korea (South),
//Kyrgyzstan,
//Laos,
//Lebanon,
//Liberia,
//Macedonia,
//Malaysia,
//Mauritius,
//Mexico,
//Mongolia,
//Mozambique,
//Namibia,
//Nepal,
//Netherlands Antilles,
//New Zealand,
//Nicaragua,
//Nigeria,
//Norway,
//Oman,
//Pakistan,
//Panama,
//Paraguay,
//Peru,
//Philippines,
//Poland,
//Qatar,
//Romania,
//Russia,
//Saint Helena,
//Saudi Arabia,
//Serbia,
//Seychelles,
//Singapore,
//Solomon Islands,
//Somalia,
//South Africa,
//Sri Lanka,
//Sweden,
//Switzerland,
//Suriname,
//Syria,
//Taiwan New,
//Thailand,
//Trinidad and Tobago,
//Turkey,
//Tuvalu,
//Ukraine,
//United Kingdom,
//United,
//Uruguay,
//Uzbekistan,
//Venezuela,
//Viet Nam,
//Yemen,
//Zimbabwe,
//
//}


//    pub fn symbol(&self) -> &'static str {
//        match self {
//            CurrCode::ALL => "Lek",
//            CurrCode::AFN => "؋",
//            CurrCode::ARS => "$",
//            CurrCode::AWG => "ƒ",
//            CurrCode::AUD => "$",
//            CurrCode::AZN => "₼",
//            CurrCode::BSD => "$",
//            CurrCode::BBD => "$",
//            CurrCode::BYN => "Br",
//            CurrCode::BZD => "BZ$",
//            CurrCode::BMD => "$",
//            CurrCode::BOB => "$b",
//            CurrCode::BAM => "KM",
//            CurrCode::BWP => "P",
//            CurrCode::BGN => "лв",
//            CurrCode::BRL => "R$",
//            CurrCode::BND => "$",
//            CurrCode::KHR => "៛",
//            CurrCode::CAD => "$",
//            CurrCode::KYD => "$",
//            CurrCode::CLP => "$",
//            CurrCode::CNY => "¥",
//            CurrCode::COP => "$",
//            CurrCode::CRC => "₡",
//            CurrCode::HRK => "kn",
//            CurrCode::CUP => "₱",
//            CurrCode::CZK => "Kč",
//            CurrCode::DKK => "kr",
//            CurrCode::DOP => "RD$",
//            CurrCode::XCD => "$",
//            CurrCode::EGP => "£",
//            CurrCode::SVC => "$",
//            CurrCode::EUR => "€",
//            CurrCode::FKP => "£",
//            CurrCode::FJD => "$",
//            CurrCode::GHS => "¢",
//            CurrCode::GIP => "£",
//            CurrCode::GTQ => "Q",
//            CurrCode::GGP => "£",
//            CurrCode::GYD => "$",
//            CurrCode::HNL => "L",
//            CurrCode::HKD => "$",
//            CurrCode::HUF => "Ft",
//            CurrCode::ISK => "kr",
//            CurrCode::INR => "",
//            CurrCode::IDR => "Rp",
//            CurrCode::IRR => "﷼",
//            CurrCode::IMP => "£",
//            CurrCode::ILS => "₪",
//            CurrCode::JMD => "J$",
//            CurrCode::JPY => "¥",
//            CurrCode::JEP => "£",
//            CurrCode::KZT => "лв",
//            CurrCode::KPW => "₩",
//            CurrCode::KRW => "₩",
//            CurrCode::KGS => "лв",
//            CurrCode::LAK => "₭",
//            CurrCode::LBP => "£",
//            CurrCode::LRD => "$",
//            CurrCode::MKD => "ден",
//            CurrCode::MYR => "RM",
//            CurrCode::MUR => "₨",
//            CurrCode::MXN => "$",
//            CurrCode::MNT => "₮",
//            CurrCode::MZN => "MT",
//            CurrCode::NAD => "$",
//            CurrCode::NPR => "₨",
//            CurrCode::ANG => "ƒ",
//            CurrCode::NZD => "$",
//            CurrCode::NIO => "C$",
//            CurrCode::NGN => "₦",
//            CurrCode::NOK => "kr",
//            CurrCode::OMR => "﷼",
//            CurrCode::PKR => "₨",
//            CurrCode::PAB => "B/.",
//            CurrCode::PYG => "Gs",
//            CurrCode::PEN => "S/.",
//            CurrCode::PHP => "₱",
//            CurrCode::PLN => "zł",
//            CurrCode::QAR => "﷼",
//            CurrCode::RON => "lei",
//            CurrCode::RUB => "₽",
//            CurrCode::SHP => "£",
//            CurrCode::SAR => "﷼",
//            CurrCode::RSD => "Дин.",
//            CurrCode::SCR => "₨",
//            CurrCode::SGD => "$",
//            CurrCode::SBD => "$",
//            CurrCode::SOS => "S",
//            CurrCode::ZAR => "R",
//            CurrCode::LKR => "₨",
//            CurrCode::SEK => "kr",
//            CurrCode::CHF => "CHF",
//            CurrCode::SRD => "$",
//            CurrCode::SYP => "£",
//            CurrCode::TWD => "NT$",
//            CurrCode::THB => "฿",
//            CurrCode::TTD => "TT$",
//            CurrCode::TRY => "₺",
//            CurrCode::TVD => "$",
//            CurrCode::UAH => "₴",
//            CurrCode::GBP => "£",
//            CurrCode::USD => "$",
//            CurrCode::UYU => "$U",
//            CurrCode::UZS => "лв",
//            CurrCode::VEF => "Bs",
//            CurrCode::VND => "₫",
//            CurrCode::YER => "﷼",
//            CurrCode::ZWD => "Z$",
//        }
//    }

