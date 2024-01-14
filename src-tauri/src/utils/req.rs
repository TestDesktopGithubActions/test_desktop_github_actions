use serde::Deserialize;
use std::usize;

use crate::utils::fun;

const DEFAULT_PAGESIZE: usize = 15;

#[derive(Debug, Clone, Deserialize)]
pub struct Params {
    pub guid: Option<String>,
    pub keyword: Option<String>,
    pub sdel: Option<i32>,
    pub status: Option<i32>,
    pub code: Option<i32>,
    pub sort: Option<String>,
    pub ip: Option<String>,
    pub page_size: Option<usize>,
    pub page: Option<usize>,
    //定义类型
    pub def: Option<i32>,
    //科目
    pub subject: Option<i32>,
    //语言
    pub l: Option<String>,
    //平台
    pub p: Option<String>,
    //时间戳
    pub t: Option<usize>,
}

impl Params {
    pub fn keyword(&self) -> String {
        self.keyword.clone().unwrap_or("".to_string())
    }

    pub fn keyword_opt(&self) -> Option<String> {
        match &self.keyword {
            Some(s) => {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            }
            _ => None,
        }
    }


    pub fn def_opt(&self) -> Option<i32> {
        match &self.def {
            Some(s) => {
                if s.is_negative() {
                    None
                } else {
                    Some(s.abs())
                }
            }
            _ => Some(2),
        }
    }




    pub fn guid_opt(&self) -> Option<String> {
        match &self.guid {
            Some(s) => {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            }
            _ => None,
        }
    }

    pub fn is_id(&self) -> bool {
        match &self.guid {
            Some(n) => fun::is_uuid(n.to_string()),
            _ => false,
        }
    }

    pub fn is_del_opt(&self) -> Option<bool> {
        match self.sdel {
            Some(n) => match n {
                1 => Some(true),
                2 => Some(false),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn sort(&self) -> String {
        self.sort.clone().unwrap_or("".to_string())
    }

    pub fn order(&self) -> Option<sea_orm::Order> {
        match self.sort().as_str() {
            "asc" => Some(sea_orm::Order::Asc),
            "desc" => Some(sea_orm::Order::Desc),
            //default parameters
            _ => Some(sea_orm::Order::Desc),
        }
    }

    pub fn page_size(&self) -> usize {
        let ps = self.page_size.unwrap_or(0);
        if ps <= 0 {
            return DEFAULT_PAGESIZE;
        }
        ps
    }

    pub fn page(&self) -> usize {
        match self.page {
            Some(s) => {
                if s == 1 {
                    0
                } else if s > 1 {
                    s - 1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn get_show_page(&self, page: usize) -> usize {
        if page == 0 {
            1
        } else {
            page + 1
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            guid: None,
            keyword: None,
            sdel: None,
            def: None,
            code: None,
            ip: None,
            subject: None,
            status: None,
            sort: None,
            page_size: None,
            page: None,
            l: None,
            p: None,
            t: None,
        }
    }
}

pub type ArticleParams = Params;
