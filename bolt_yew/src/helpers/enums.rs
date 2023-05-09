// use bolt_ws::prelude::HttpMethod;

pub enum RequestTabs {
    Body,
    Params,
    Headers,
}

impl From<u8> for RequestTabs {
    fn from(value: u8) -> Self {
        match value {
            1 => RequestTabs::Body,
            2 => RequestTabs::Params,
            3 => RequestTabs::Headers,
            _ => panic!("Invalid value for RequestTabs"),
        }
    }
}

impl From<RequestTabs> for u8 {
    fn from(tab: RequestTabs) -> Self {
        match tab {
            RequestTabs::Body => 1,
            RequestTabs::Params => 2,
            RequestTabs::Headers => 3,
        }
    }
}

pub enum ResponseTabs {
    Body,
    Headers,
}

impl From<u8> for ResponseTabs {
    fn from(value: u8) -> Self {
        match value {
            1 => ResponseTabs::Body,
            2 => ResponseTabs::Headers,
            _ => panic!("Invalid value for ResponseTabs"),
        }
    }
}

impl From<ResponseTabs> for u8 {
    fn from(tab: ResponseTabs) -> Self {
        match tab {
            ResponseTabs::Body => 1,
            ResponseTabs::Headers => 2,
        }
    }
}
