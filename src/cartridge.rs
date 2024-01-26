use std::{arch::x86_64::_SIDD_LEAST_SIGNIFICANT, result};

pub struct RomHeader { 
    entry: Vec<u8>,
    logo: Vec<u8>,

    title: String,
    new_lic_code: u16,
    sgb_flag: u8,
    rom_type: u8,
    ram_size: u8,
    dest_code: u8,
    lic_code: u8,
    version: u8,
    checksum: u8,
    global_checksum: u16
}

pub struct CartContext {
    filename: String,
    rom_size: u32,
    rom_data: u8,
    header: RomHeader
}

pub fn get_lic_code(lic_code: u8) -> String { //returns the liscense name of the given liscense code
    let mut lic_codes: Vec<u8> = vec!();
    let mut lic_names: Vec<&str> = vec!();
    //fucking kill me
    lic_codes.push(0x00);  lic_names.push("None");
    lic_codes.push(0x01);  lic_names.push("Nintendo R&D1");
    lic_codes.push(0x08);  lic_names.push("Capcom");
    lic_codes.push(0x13);  lic_names.push("Electronic Arts");
    lic_codes.push(0x18);  lic_names.push("Hudson Soft");
    lic_codes.push(0x19);  lic_names.push("b-ai");
    lic_codes.push(0x20);  lic_names.push("kss");
    lic_codes.push(0x22);  lic_names.push("pow");
    lic_codes.push(0x24);  lic_names.push("PCM Complete");
    lic_codes.push(0x25);  lic_names.push("san-x");
    lic_codes.push(0x28);  lic_names.push("Kemco Japan");
    lic_codes.push(0x29);  lic_names.push("seta");
    lic_codes.push(0x30);  lic_names.push("Viacom");
    lic_codes.push(0x31);  lic_names.push("Nintendo");
    lic_codes.push(0x32);  lic_names.push("Bandai");
    lic_codes.push(0x33);  lic_names.push("Ocean/Acclaim");
    lic_codes.push(0x34);  lic_names.push("Konami");
    lic_codes.push(0x35);  lic_names.push("Hector");
    lic_codes.push(0x37);  lic_names.push("Taito");
    lic_codes.push(0x38);  lic_names.push("Hudson");
    lic_codes.push(0x39);  lic_names.push("Banpresto");
    lic_codes.push(0x41);  lic_names.push("Ubi Soft");
    lic_codes.push(0x42);  lic_names.push("Atlus");
    lic_codes.push(0x44);  lic_names.push("Malibu");
    lic_codes.push(0x46);  lic_names.push("angel");
    lic_codes.push(0x47);  lic_names.push("Bullet-Proof");
    lic_codes.push(0x49);  lic_names.push("irem");
    lic_codes.push(0x50);  lic_names.push("Absolute");
    lic_codes.push(0x51);  lic_names.push("Acclaim");
    lic_codes.push(0x52);  lic_names.push("Activision");
    lic_codes.push(0x53);  lic_names.push("American Sammy");
    lic_codes.push(0x54);  lic_names.push("Konami");
    lic_codes.push(0x55);  lic_names.push("Hi tech entertainment");
    lic_codes.push(0x56);  lic_names.push("LJN");
    lic_codes.push(0x57);  lic_names.push("Matchbox");
    lic_codes.push(0x58);  lic_names.push("Mattel");
    lic_codes.push(0x59);  lic_names.push("Mitton Bradley");
    lic_codes.push(0x60);  lic_names.push("Titus");
    lic_codes.push(0x61);  lic_names.push("Virgin");
    lic_codes.push(0x64);  lic_names.push("LucasArts");
    lic_codes.push(0x67);  lic_names.push("Ocean");
    lic_codes.push(0x69);  lic_names.push("Electronic Arts");
    lic_codes.push(0x70);  lic_names.push("Infogrames");
    lic_codes.push(0x71);  lic_names.push("Interplay");
    lic_codes.push(0x72);  lic_names.push("Broderbund");
    lic_codes.push(0x73);  lic_names.push("sculptured");
    lic_codes.push(0x75);  lic_names.push("sci");
    lic_codes.push(0x78);  lic_names.push("THQ");
    lic_codes.push(0x79);  lic_names.push("Accolade");
    lic_codes.push(0x80);  lic_names.push("misawa");
    lic_codes.push(0x83);  lic_names.push("lozc");
    lic_codes.push(0x86);  lic_names.push("Tokuma Shoten Intermedia");
    lic_codes.push(0x87);  lic_names.push("Tsukada Original");
    lic_codes.push(0x91);  lic_names.push("Chunsoft");
    lic_codes.push(0x92);  lic_names.push("Video system");
    lic_codes.push(0x93);  lic_names.push("Ocean/Acclaim");
    lic_codes.push(0x95);  lic_names.push("Varie");
    lic_codes.push(0x96);  lic_names.push("Yonezawa/s'pal");
    lic_codes.push(0x97);  lic_names.push("Kaneko");
    lic_codes.push(0x99);  lic_names.push("Pack in soft");
    lic_codes.push(0xA4);  lic_names.push("Bottom Up");


    for (i, index) in lic_codes.iter().enumerate() {
        if index == &lic_code {
            return String::from(lic_names[i])
        }
    }

    return String::from("UNKNOWN")
}