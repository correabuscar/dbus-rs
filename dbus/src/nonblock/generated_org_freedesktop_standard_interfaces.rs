// This code was autogenerated with `dbus-codegen-rust -m none -c nonblock -g -i org.freedesktop.DBus --dbuscrate crate --file ./data/standard_interfaces.xml -o ../dbus/src/nonblock/generated_org_freedesktop_standard_interfaces.rs`, see https://github.com/diwic/dbus-rs
use crate as dbus;
#[allow(unused_imports)]
use crate::arg;
use crate::nonblock;

pub trait Properties {
    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> nonblock::MethodReply<R0>;
    fn get_all(&self, interface_name: &str) -> nonblock::MethodReply<arg::PropMap>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> nonblock::MethodReply<()>;
}

#[derive(Debug)]
pub struct PropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for PropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for PropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(PropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for PropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> Properties for nonblock::Proxy<'a, C> {

    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> nonblock::MethodReply<R0> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<R0>, )| Ok((r.0).0, ))
    }

    fn get_all(&self, interface_name: &str) -> nonblock::MethodReply<arg::PropMap> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (arg::PropMap, )| Ok(r.0, ))
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, arg::Variant(value), ))
    }
}

pub trait Introspectable {
    fn introspect(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> Introspectable for nonblock::Proxy<'a, C> {

    fn introspect(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait Peer {
    fn ping(&self) -> nonblock::MethodReply<()>;
    fn get_machine_id(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> Peer for nonblock::Proxy<'a, C> {

    fn ping(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait ObjectManager {
    fn get_managed_objects(&self) -> nonblock::MethodReply<::std::collections::HashMap<dbus::Path<'static>, ::std::collections::HashMap<String, arg::PropMap>>>;
}

#[derive(Debug)]
pub struct ObjectManagerInterfacesAdded {
    pub object: dbus::Path<'static>,
    pub interfaces: ::std::collections::HashMap<String, arg::PropMap>,
}

impl arg::AppendAll for ObjectManagerInterfacesAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.object, i);
        arg::RefArg::append(&self.interfaces, i);
    }
}

impl arg::ReadAll for ObjectManagerInterfacesAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ObjectManagerInterfacesAdded {
            object: i.read()?,
            interfaces: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ObjectManagerInterfacesAdded {
    const NAME: &'static str = "InterfacesAdded";
    const INTERFACE: &'static str = "org.freedesktop.DBus.ObjectManager";
}

#[derive(Debug)]
pub struct ObjectManagerInterfacesRemoved {
    pub object: dbus::Path<'static>,
    pub interfaces: Vec<String>,
}

impl arg::AppendAll for ObjectManagerInterfacesRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.object, i);
        arg::RefArg::append(&self.interfaces, i);
    }
}

impl arg::ReadAll for ObjectManagerInterfacesRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ObjectManagerInterfacesRemoved {
            object: i.read()?,
            interfaces: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ObjectManagerInterfacesRemoved {
    const NAME: &'static str = "InterfacesRemoved";
    const INTERFACE: &'static str = "org.freedesktop.DBus.ObjectManager";
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> ObjectManager for nonblock::Proxy<'a, C> {

    fn get_managed_objects(&self) -> nonblock::MethodReply<::std::collections::HashMap<dbus::Path<'static>, ::std::collections::HashMap<String, arg::PropMap>>> {
        self.method_call("org.freedesktop.DBus.ObjectManager", "GetManagedObjects", ())
            .and_then(|r: (::std::collections::HashMap<dbus::Path<'static>, ::std::collections::HashMap<String, arg::PropMap>>, )| Ok(r.0, ))
    }
}
