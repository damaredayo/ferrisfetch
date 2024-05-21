use crate::State;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl State {
    pub fn fetch_gpu(&self) -> Vec<String> {
        vk_gpu().unwrap_or_else(find_gpu)
    }
}

fn vk_gpu() -> Option<Vec<String>> {
    let mut gpus = Vec::new();

    unsafe {
        let entry = ash::Entry::load().ok()?;
        let instance_info = ash::vk::InstanceCreateInfo::default();

        let instance = entry.create_instance(&instance_info, None).ok()?; // blehhh this takes too long

        for physical_device in instance.enumerate_physical_devices().ok()? {
            let properties = instance.get_physical_device_properties(physical_device);

            let props = properties
                .device_name
                .iter()
                .map(|&c| c as u8 as char)
                .collect::<String>()
                .into();

            gpus.push(props);
        }
    }

    Some(gpus)
}

fn find_gpu() -> Vec<String> {
    let mut gpus = Vec::new();

    let _ = nvidia_gpu(&mut gpus);
    let _ = amd_gpu(&mut gpus);
    let _ = intel_gpu(&mut gpus);

    gpus
}

fn nvidia_gpu(gpus: &mut Vec<String>) -> Result<()> {
    let nvml = nvml_wrapper::Nvml::init()?;

    let device_count = nvml.device_count()?;
    for i in 0..device_count {
        let device = nvml.device_by_index(i)?;
        let name = device.name()?;
        gpus.push(name);
    }

    Ok(())
}

fn amd_gpu(gpus: &mut Vec<String>) -> Result<()> {
    let mut rocm = rocm_smi_lib::RocmSmi::init().map_err(|e| e.to_string())?;

    let devices_count = rocm.get_device_count();
    for i in 0..devices_count {
        let device = rocm.get_device_identifiers(i).map_err(|e| e.to_string())?;
        gpus.push(device.name.map_err(|e| e.to_string())?);
    }

    Ok(())
}

fn intel_gpu(_gpus: &mut Vec<String>) -> Result<()> {
    Ok(())
}
