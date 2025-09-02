pub trait SendableComponent<TSendable>
where
    Self: Sized,
{
    fn to_sendable(&self) -> TSendable;
    fn from_sendable(sendable: TSendable) -> Option<Self>;
}
