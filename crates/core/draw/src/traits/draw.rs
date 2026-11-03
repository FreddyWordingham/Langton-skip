pub trait Draw: Sized + 'static {
    const ALL: &'static [Self];

    fn name(&self) -> &'static str;

    fn menu_sprite_path(&self) -> &'static str;
}
