#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

fn main() -> windows_core::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let _com_releaser = ComReleaser::new()?;
    if args.len() >= 2 {
        let id = windows_core::HSTRING::from(&args[1]);
        set_default_device(&id)?;
        println!("{}", id);
    } else {
        let devices_render = list_devices(windows::Win32::Media::Audio::eRender)?;
        for device in devices_render {
            println!("eRender  | {} | {} | {}", if device.is_default { "Default" } else { "-      " }, device.id, device.friendly_name);
        }
        let devices_capture = list_devices(windows::Win32::Media::Audio::eCapture)?;
        for device in devices_capture {
            println!("eCapture | {} | {} | {}", if device.is_default { "Default" } else { "-      " }, device.id, device.friendly_name);
        }
    }
    Ok(())
}

pub fn set_default_device<P0: windows_core::Param<windows_core::PCWSTR>>(device_id: P0) -> windows_core::Result<()> {
    unsafe {
        let policy_config: IPolicyConfig = windows::Win32::System::Com::CoCreateInstance(&CLSID_CPolicyConfigClient, None, windows::Win32::System::Com::CLSCTX_ALL)?;
        policy_config.SetDefaultEndpoint(device_id, windows::Win32::Media::Audio::eConsole)
    }
}

pub struct DeviceItem {
    id: windows_core::HSTRING,
    friendly_name: windows_core::HSTRING,
    is_default: bool,
}

pub fn list_devices(dataflow: windows::Win32::Media::Audio::EDataFlow) -> windows_core::Result<Vec<DeviceItem>> {
    unsafe {
        let enumerator = windows::Win32::System::Com::CoCreateInstance::<_, windows::Win32::Media::Audio::IMMDeviceEnumerator>(&windows::Win32::Media::Audio::MMDeviceEnumerator, None, windows::Win32::System::Com::CLSCTX_ALL)?;
        let default_device = enumerator.GetDefaultAudioEndpoint(dataflow, windows::Win32::Media::Audio::eConsole)?;
        let default_device_id = default_device.GetId()?.to_hstring();
        let collection = enumerator.EnumAudioEndpoints(dataflow, windows::Win32::Media::Audio::DEVICE_STATE_ACTIVE)?;
        let count = collection.GetCount()?;
        let mut devices = Vec::new();
        for i in 0..count {
            let device = collection.Item(i)?;
            let id = device.GetId()?.to_hstring();
            let is_default = id.to_os_string() == default_device_id.to_os_string();
            let property_store = device.OpenPropertyStore(windows::Win32::System::Com::STGM_READ)?;
            let mut prop_variant = property_store.GetValue(&windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName)?;
            let friendly_name = prop_variant.Anonymous.Anonymous.Anonymous.pwszVal.to_hstring();
            devices.push(DeviceItem {
                id,
                friendly_name,
                is_default,
            });
            windows::Win32::System::Com::StructuredStorage::PropVariantClear(&mut prop_variant)?;
        }
        Ok(devices)
    }
}

// region CoInitializeEx & CoUninitialize

struct ComReleaser;

impl ComReleaser {
    pub fn new() -> windows::core::Result<Self> {
        unsafe {
            windows::Win32::System::Com::CoInitializeEx(None, windows::Win32::System::Com::COINIT_APARTMENTTHREADED).ok()?;
            Ok(Self {})
        }
    }
}

impl Drop for ComReleaser {
    fn drop(&mut self) {
        unsafe {
            windows::Win32::System::Com::CoUninitialize();
        }
    }
}

// endregion

// region IPolicyConfig

windows_core::imp::define_interface!(IPolicyConfig, IPolicyConfig_Vtbl, 0xf8679f50_850a_41cf_9c72_430f290290c8);
windows_core::imp::interface_hierarchy!(IPolicyConfig, windows_core::IUnknown);
impl IPolicyConfig {
    pub unsafe fn SetDefaultEndpoint<P0>(&self, device_id: P0, role: windows::Win32::Media::Audio::ERole) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultEndpoint)(windows_core::Interface::as_raw(self), device_id.param().abi(), role).ok() }
    }
}
#[repr(C)]
pub struct IPolicyConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMixFormat: unsafe extern "system" fn() -> !,
    pub GetDeviceFormat: unsafe extern "system" fn() -> !,
    pub ResetDeviceFormat: unsafe extern "system" fn() -> !,
    pub SetDeviceFormat: unsafe extern "system" fn() -> !,
    pub GetProcessingPeriod: unsafe extern "system" fn() -> !,
    pub SetProcessingPeriod: unsafe extern "system" fn() -> !,
    pub GetShareMode: unsafe extern "system" fn() -> !,
    pub SetShareMode: unsafe extern "system" fn() -> !,
    pub GetPropertyValue: unsafe extern "system" fn() -> !,
    pub SetPropertyValue: unsafe extern "system" fn() -> !,
    pub SetDefaultEndpoint: unsafe extern "system" fn(this: *mut core::ffi::c_void, device_id: windows_core::PCWSTR, role: windows::Win32::Media::Audio::ERole) -> windows_core::HRESULT,
    pub SetEndpointVisibility: unsafe extern "system" fn() -> !,
}

pub const CLSID_CPolicyConfigClient: windows_core::GUID = windows_core::GUID::from_u128(0x870af99c_171d_4f9e_af0d_e63df40c2bc9);

// endregion