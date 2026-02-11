use std::sync::RwLock;

use imagesize::ImageSize;

static INPUT_IMG_HDR_SMALL: RwLock<[u8; 1024]> = RwLock::new([0; 1024]);

static OUTPUT_IMG_DIM: RwLock<ImageSize> = RwLock::new(ImageSize {
    width: 0,
    height: 0,
});

fn input_img_hdr_small_ptr_raw() -> Option<*mut u8> {
    let mut oguard = INPUT_IMG_HDR_SMALL.try_write().ok();
    let oa: Option<&mut [u8; 1024]> = oguard.as_deref_mut();
    oa.map(|a| a.as_mut_ptr())
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn input_img_hdr_small_ptr() -> *mut u8 {
    input_img_hdr_small_ptr_raw().unwrap_or(std::ptr::null_mut())
}

fn bytes2size(dat: &[u8]) -> Option<ImageSize> {
    imagesize::blob_size(dat).ok()
}

fn guess_size_raw() -> Option<ImageSize> {
    let oguard = INPUT_IMG_HDR_SMALL.try_read().ok();
    let oa: Option<&[u8; 1024]> = oguard.as_deref();
    oa.and_then(|a| bytes2size(&a[..]))
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn guess_size() -> i32 {
    let osz: Option<ImageSize> = guess_size_raw();
    let mut guard = OUTPUT_IMG_DIM.try_write().ok();
    let oi: Option<&mut ImageSize> = guard.as_deref_mut();
    match (osz, oi) {
        (Some(sz), Some(i)) => {
            *i = sz;
            0
        }
        _ => -1,
    }
}

fn sz2width(sz: ImageSize) -> u32 {
    let width: usize = sz.width;
    width.try_into().ok().unwrap_or_default()
}

fn sz2height(sz: ImageSize) -> u32 {
    let height: usize = sz.height;
    height.try_into().ok().unwrap_or_default()
}

fn width_raw() -> u32 {
    let oguard = OUTPUT_IMG_DIM.try_read().ok();
    let oi: Option<ImageSize> = oguard.as_deref().copied();
    let ow: Option<u32> = oi.map(sz2width);
    ow.unwrap_or_default()
}

fn height_raw() -> u32 {
    let oguard = OUTPUT_IMG_DIM.try_read().ok();
    let oi: Option<ImageSize> = oguard.as_deref().copied();
    let oh: Option<u32> = oi.map(sz2height);
    oh.unwrap_or_default()
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn width() -> u32 {
    width_raw()
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn height() -> u32 {
    height_raw()
}
