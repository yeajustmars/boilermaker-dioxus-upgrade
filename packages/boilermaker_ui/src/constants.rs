use dioxus::prelude::*;
pub const DROPDOWN_MENU_STYLE: &str = "absolute left-0 top-12 w-48 bg-white dark:bg-neutral-900 rounded shadow-lg border border-l-0 border-t-0 border-neutral-300 dark:border-neutral-700 z-10 text-sm ";

pub const DROPDOWN_LINK_STYLE: &str =
    "block px-4 py-2 hover:bg-neutral-100 dark:hover:bg-neutral-700";

pub const INDENTED_DROPDOWN_LINK_STYLE: &str =
    "block px-8 py-2 hover:bg-neutral-100 dark:hover:bg-neutral-700";

pub const FAVICON: Asset = asset!("/assets/img/logo-flame.png");

pub const NAVBAR_STYLE: &str = "flex flex-row space-x-4 p-2 items-center justify-between bg-gradient-to-b from-white to-neutral-100 dark:from-neutral-800 dark:to-neutral-900 border-b border-solid border-neutral-300 dark:border-neutral-950 text-neutral-600 dark:text-neutral-300";
