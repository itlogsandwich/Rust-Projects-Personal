use rand_derive2::RandGen;
use rand::random;
use rand::prelude::*;
use inquire::Select;




fn main() 
{
    #[derive(Debug, RandGen, Display)]
    enum Name
    {
        Akira,      California, Daedalus,
        Eisenberg,  Interpid,   Miranda,      
        Nova,       Reliant,    Sagan
    }

    let a_name: Name = rand::random();

    #[derive(Debug, RandGen)]
    struct Station
    {
        name: Name,
        version: u8,
        sections: Vec<Section>
    }

    impl Station 
    {
        fn new() -> Self
        {
            Station
            {
                name: random(),
                version: random(),
                sections:(0..10)
                    .map(|_| random())
                    .collect()
            }
        }
    }

    let rand_int: u8 = random();

    let rand_ch: char = random();

    let rand_int: f32 = random();

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rand::thread_rng());

    #[derive(Debug, RandGen, Eq, PartialEq)]
    struct Section
    {
        name: SectionName,
        active: bool,
    }

    #[derive(Debug, RandGen, Display, EnumString)]
    #[derive(Eq, PartialEq)]
    enum SectionName
    {
        AstroScience,       Solar,          Antenna,
        RadiationMirros,    Sleeping,       NuclearGenerator,
        Galley,             Transponder,    Tracking
    }

    fn menu(items: &[String]) -> String
    {
        Select::new("MENU", items.to_vec())
        .prompt()
        .unwrap()
    }

    impl Station 
    {
        fn days_left(&self) -> usize
        {
            self.sections
                .iter()
                .filter(|m| m.active)
                .count()
        }
    }

    impl Station
    {
        fn working_sections(&self) -> Vec<String> 
        {
            self.sections.iter()
                .filter(|m| m.active)
                .map(|m| m.name.to_string())
                .collect()
        }
        fn broken_sections(&self) -> Vec<String>
        {
            self.sections.iter() 
                .filter(|m| !m.active)
                .map(|m| m.name.to_string())
                .collect()
        }
    }

    fn repair(broken_section: String, station: &mut Station)
    {
        let section = SectionName::from(
            broken_section.as_str()).unwrap();

        let broken_index = station.sections.iter()
            .position(|m| m.name == section)
            .expect("Section not found.");

        station.sections[broken_index].active = true;
    }
}
