// Embeds the app icon and version details in the Windows .exe. Other platforms need nothing here.
fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=assets/icon.ico");
        let mut resource = winresource::WindowsResource::new();
        resource
            .set_icon("assets/icon.ico")
            .set("ProductName", "Clip5")
            .set("FileDescription", "Clip5");
        resource
            .compile()
            .expect("couldn't embed the Windows icon (is the Windows SDK installed?)");
    }
}
