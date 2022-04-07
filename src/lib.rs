use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

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

impl From<IpAddr> for IpFamily {
    fn from(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(_) => IpFamily::V4,
            IpAddr::V6(_) => IpFamily::V6,
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

impl From<SocketAddr> for IpFamily {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(_) => IpFamily::V4,
            SocketAddr::V6(_) => IpFamily::V6,
        }
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
