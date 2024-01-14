pub(crate) trait Pack {
    fn pack_one(tag: &str, data: Option<super::payload::Payload>) -> Self;
}

pub(crate) trait Deliver: Pack {
    fn send(self) -> Result<(), crate::Error>;
}
