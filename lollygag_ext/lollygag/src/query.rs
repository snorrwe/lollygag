pub enum HtmlQuery {
    None,
    Attribute {
        key: String,
        value: String,
    },
    Name(String),
    Data(String),
    And {
        x: Box<HtmlQuery>,
        y: Box<HtmlQuery>,
    },
    Or {
        x: Box<HtmlQuery>,
        y: Box<HtmlQuery>,
    },
    Child(Box<HtmlQuery>),
    Sibling(Box<HtmlQuery>),
}

impl HtmlQuery {
    pub fn and(self, query: HtmlQuery) -> HtmlQuery {
        match self {
            _ => HtmlQuery::And {
                x: Box::new(self),
                y: Box::new(query),
            },
            HtmlQuery::None => return HtmlQuery::None,
        }
    }

    pub fn or(self, query: HtmlQuery) -> HtmlQuery {
        match self {
            _ => HtmlQuery::Or {
                x: Box::new(self),
                y: Box::new(query),
            },
            HtmlQuery::None => return query,
        }
    }
}
