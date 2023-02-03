use std::{
    fmt::{Debug, Display},
    hash::Hash,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs},
    str::FromStr,
};

mod sealed {
    pub trait Sealed {}
}

pub trait AnyIpFamily
where
    Self: sealed::Sealed,
    Self::Addr: IpFamilyAddr<Family = Self>,
    Self::SocketAddr: IpFamilySocketAddr<Family = Self>,
    IpAddr: From<Self::Addr>,
    SocketAddr: From<Self::SocketAddr>,
{
    type Addr;
    type SocketAddr;

    const FAMILY: IpFamily;
}

pub trait IpFamilyAddr
where
    Self: sealed::Sealed
        + Copy
        + Debug
        + Display
        + Eq
        + From<Self::Raw>
        + From<Self::Bytes>
        + FromStr
        + Hash
        + Ord,
    Self::Family: AnyIpFamily<Addr = Self>,
    Self::Raw: From<Self>,
    IpAddr: From<Self>,
    SocketAddr: From<<Self::Family as AnyIpFamily>::SocketAddr>,
{
    type Family;
    type Raw;
    type Bytes;

    const BYTES: usize;
    const LOCALHOST: Self;
    const UNSPECIFIED: Self;

    fn octets(&self) -> Self::Bytes;
    fn is_unspecified(&self) -> bool;
    fn is_loopback(&self) -> bool;
    fn is_multicast(&self) -> bool;
}

pub trait IpFamilySocketAddr
where
    Self: sealed::Sealed + Copy + Debug + Display + Eq + FromStr + Hash + Ord + ToSocketAddrs,

    Self::Family: AnyIpFamily<SocketAddr = Self>,
    IpAddr: From<<Self::Family as AnyIpFamily>::Addr>,
    SocketAddr: From<Self>,
{
    type Family;

    fn new(ip: <Self::Family as AnyIpFamily>::Addr, port: u16) -> Self;
    fn ip(&self) -> <Self::Family as AnyIpFamily>::Addr;
    fn set_ip(&mut self, new_ip: <Self::Family as AnyIpFamily>::Addr);
    fn port(&self) -> u16;
    fn set_port(&mut self, new_port: u16);
}

pub struct IpFamilyV4;

impl sealed::Sealed for IpFamilyV4 {}
impl AnyIpFamily for IpFamilyV4 {
    type Addr = Ipv4Addr;

    type SocketAddr = SocketAddrV4;

    const FAMILY: IpFamily = IpFamily::V4;
}

const IPV4_ADDR_BYTES: usize = 4;
impl sealed::Sealed for Ipv4Addr {}
impl IpFamilyAddr for Ipv4Addr {
    type Family = IpFamilyV4;
    type Raw = u32;
    type Bytes = [u8; IPV4_ADDR_BYTES];

    const BYTES: usize = IPV4_ADDR_BYTES;
    const LOCALHOST: Self = Self::LOCALHOST;
    const UNSPECIFIED: Self = Self::UNSPECIFIED;

    fn octets(&self) -> Self::Bytes {
        self.octets()
    }

    fn is_unspecified(&self) -> bool {
        self.is_unspecified()
    }

    fn is_loopback(&self) -> bool {
        self.is_loopback()
    }

    fn is_multicast(&self) -> bool {
        self.is_multicast()
    }
}

impl sealed::Sealed for SocketAddrV4 {}
impl IpFamilySocketAddr for SocketAddrV4 {
    type Family = IpFamilyV4;

    fn new(ip: <Self::Family as AnyIpFamily>::Addr, port: u16) -> Self {
        Self::new(ip, port)
    }

    fn ip(&self) -> <Self::Family as AnyIpFamily>::Addr {
        *self.ip()
    }

    fn set_ip(&mut self, new_ip: <Self::Family as AnyIpFamily>::Addr) {
        self.set_ip(new_ip)
    }

    fn port(&self) -> u16 {
        self.port()
    }

    fn set_port(&mut self, new_port: u16) {
        self.set_port(new_port)
    }
}

pub struct IpFamilyV6;

impl sealed::Sealed for IpFamilyV6 {}
impl AnyIpFamily for IpFamilyV6 {
    type Addr = Ipv6Addr;

    type SocketAddr = SocketAddrV6;

    const FAMILY: IpFamily = IpFamily::V6;
}

const IPV6_ADDR_BYTES: usize = 16;
impl sealed::Sealed for Ipv6Addr {}
impl IpFamilyAddr for Ipv6Addr {
    type Family = IpFamilyV6;
    type Raw = u128;
    type Bytes = [u8; IPV6_ADDR_BYTES];

    const BYTES: usize = IPV6_ADDR_BYTES;
    const LOCALHOST: Self = Self::LOCALHOST;
    const UNSPECIFIED: Self = Self::UNSPECIFIED;

    fn octets(&self) -> Self::Bytes {
        self.octets()
    }

    fn is_unspecified(&self) -> bool {
        self.is_unspecified()
    }

    fn is_loopback(&self) -> bool {
        self.is_loopback()
    }

    fn is_multicast(&self) -> bool {
        self.is_multicast()
    }
}

impl sealed::Sealed for SocketAddrV6 {}
impl IpFamilySocketAddr for SocketAddrV6 {
    type Family = IpFamilyV6;

    fn new(ip: <Self::Family as AnyIpFamily>::Addr, port: u16) -> Self {
        Self::new(ip, port, 0, 0)
    }

    fn ip(&self) -> <Self::Family as AnyIpFamily>::Addr {
        *self.ip()
    }

    fn set_ip(&mut self, new_ip: <Self::Family as AnyIpFamily>::Addr) {
        self.set_ip(new_ip)
    }

    fn port(&self) -> u16 {
        self.port()
    }

    fn set_port(&mut self, new_port: u16) {
        self.set_port(new_port)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IpFamily {
    V4,
    V6,
}

impl IpFamily {
    pub const fn localhost(&self) -> IpAddr {
        match self {
            Self::V4 => IpAddr::V4(Ipv4Addr::LOCALHOST),
            Self::V6 => IpAddr::V6(Ipv6Addr::LOCALHOST),
        }
    }

    pub fn unspecified(&self) -> IpAddr {
        match self {
            Self::V4 => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            Self::V6 => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
        }
    }
}

impl AsRef<IpFamily> for IpAddr {
    fn as_ref(&self) -> &IpFamily {
        match self {
            IpAddr::V4(_) => &IpFamily::V4,
            IpAddr::V6(_) => &IpFamily::V6,
        }
    }
}

impl AsRef<IpFamily> for SocketAddr {
    fn as_ref(&self) -> &IpFamily {
        match self {
            SocketAddr::V4(_) => &IpFamily::V4,
            SocketAddr::V6(_) => &IpFamily::V6,
        }
    }
}

impl<T: AsRef<IpFamily>> From<T> for IpFamily {
    fn from(value: T) -> Self {
        *value.as_ref()
    }
}

pub trait IpFamilyExt {
    fn family(&self) -> IpFamily;
}

impl<T: AsRef<IpFamily>> IpFamilyExt for T {
    fn family(&self) -> IpFamily {
        *self.as_ref()
    }
}
