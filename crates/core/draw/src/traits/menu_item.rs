pub trait MenuItem: Sized + 'static {
    const ALL: &'static [Self];

    fn label(&self) -> &'static str;

    fn menu_sprite_path(&self) -> &'static str;
}
