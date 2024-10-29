use std::path::PathBuf;

use crate::sheet::{Error, SheetCreatable, SheetReadable, SheetWritable};

pub fn read<S>() -> Result<(S, PathBuf), Error>
where
    S: SheetReadable,
{
    let mut path = rfd::FileDialog::new()
        .set_title("Which sheet do you want to pick up?")
        .add_filter(S::EXTENSION, &[S::EXTENSION])
        .pick_file()
        .ok_or(Error::not_selected())?;
    let sheet = S::read(&mut path)?;
    Ok((sheet, path))
}

pub fn write<S>(sheet: &S) -> Result<PathBuf, Error>
where
    S: SheetWritable,
{
    let mut path = rfd::FileDialog::new()
        .set_title("Where should we store this sheet?")
        .add_filter(S::EXTENSION, &[S::EXTENSION])
        .save_file()
        .ok_or(Error::not_selected())?;
    sheet.write(&mut path)?;
    Ok(path)
}

pub fn create<S>() -> Result<(S, PathBuf), Error>
where
    S: SheetCreatable,
{
    let mut path = rfd::FileDialog::new()
        .set_title("Where should we store this sheet?")
        .add_filter(S::EXTENSION, &[S::EXTENSION])
        .save_file()
        .ok_or(Error::not_selected())?;
    let sheet = S::create(&mut path)?;
    Ok((sheet, path))
}

pub fn open_or_create<S>() -> Result<(S, PathBuf), Error>
where
    S: SheetCreatable + SheetReadable,
{
    let message = match rfd::MessageDialog::new()
        .set_buttons(rfd::MessageButtons::YesNoCancelCustom(
            "Open".to_string(),
            "Create".to_string(),
            "Cancel".to_string(),
        ))
        .set_title("Sheet")
        .set_description("Want to open or create a sheet?")
        .show()
    {
        rfd::MessageDialogResult::Custom(message) => message,
        _ => return Err(Error::not_selected()),
    };

    match message.as_str() {
        "Open" => read(),
        "Create" => create(),
        _ => Err(Error::not_selected()),
    }
}
