use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::util::tree::ImmutableTree;
use anyhow::*;

pub mod content;
pub mod util;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

pub fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new().clear_filter_ignore().build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    let content = Path::new("../content");

    if !content.exists() {
        bail!("invalid path to content: {}", content.canonicalize()?.display());
    }

    info!("content path is {:?}", content.canonicalize());

    if !content.is_dir() {
        bail!("content path isn't a directory");
    }

    let out = Path::new("../out");

    info!("Out path is: {:?}", out.canonicalize()?.display());

    let sites_folder = content.join("sites");

    if !sites_folder.exists() {
        bail!("sites folder didn't exist in content: {}", sites_folder.canonicalize()?.display());
    }

    if !sites_folder.is_dir() {
        bail!(
            "sites folder exists but isn't a directory: {}",
            sites_folder.canonicalize()?.display()
        );
    }

    info!("sites folder found at {}", sites_folder.canonicalize()?.display());

    info!("generating sites tree");

    let mut sites_tree = ImmutableTree::<PathBuf>::new(std::fs::read_dir(sites_folder)?.filter_map(|v| v.map(|v| v.path()).ok()));

    sites_tree.add_layers_recursively(|path| {
        if !path.is_dir() {
            return None;
        }

        if let std::result::Result::Ok(entry) = std::fs::read_dir(path) {
            Some(entry.filter_map(|v| v.ok().map(|v| v.path()).filter(|v| v.is_dir())))
        } else {
            None
        }
    });

    if sites_tree.is_empty() {
        bail!("site tree is empty");
    }

    info!("sites tree is generated as:\n{:?}", sites_tree);

    info!("Generating html string");

    let meta_str = |depth| {
        format!(
            concat!(
                "<!DOCTYPE html>\n",
                "<html>\n",
                "<head>\n",
                "    <title>Rewrites Are Bliss</title>\n",
                "    <meta content=\"width=device-width, initial-scale=1.0\" name=\"viewport\">\n",
                "    <link rel=\"stylesheet\" type=\"text/css\" href=\"{dots}styles/default.css\"/>\n",
                "    <link rel=\"stylesheet\" href=\"{dots}styles/codetheme.css\">\n",
                "    <script src=\"https://kit.fontawesome.com/d7fa8be03e.js\" crossorigin=\"anonymous\"></script>\n",
                "    <script src=\"{dots}vendor/highlight/highlight.min.js\"></script>\n",
                "    <script src=\"{dots}scripts/main.js\"></script>\n",
                "</head>\n"
            ),
            dots = "../".repeat(depth + 4)
        )
    };

    let pre_header = concat!(
        "<body>\n",
        "    <div id=\"page\">\n",
        "        <div id=\"header\">\n",
        "            <p id=\"header_title_label\">Rewrites Are Bliss</p>\n",
        "            <div id=\"header_container\">\n"
    );

    let mut header = String::with_capacity(8192);

    for i in 0..sites_tree.root_count() {
        let root = sites_tree
            .get(i)
            .ok_or(anyhow!("sites tree contained fewer items than root_count"))?
            .val();

        write!(
            header,
            concat!(
                "            <div class=\"tl_menu_item\">\n",
                "               <p>{}</p>\n",
                "            </div>\n"
            ),
            capitalise(
                root.file_name()
                    .map(|v| v.to_str())
                    .flatten()
                    .ok_or(anyhow!("unable to obtain file name from site: {}", root.canonicalize()?.display()))?
            )
        )?;
    }

    let post_header = concat!(
        "            </div>\n",
        "            <div id=\"search_input\">\n",
        "                <input placeholder=\"Search content...\" id=\"search_input_field\"/>\n",
        "                <span id=\"search_input_icon\" class=\"fa-solid fa-search\"></span>\n",
        "            </div>\n",
        "        </div>\n",
    );

    let pre_content = concat!("        <div id=\"content\">\n");

    let post_content = concat!(
        "        </div>\n",
        "        <script>\n",
        "            hljs.highlightAll();\n",
        "        </script>\n",
        "    </body>\n",
        "</html>\n",
    );

    let definitions = content.join("definitions");

    info!("generating out directory");

    if out.exists() {
        info!("out directory already exists, removing previous installation");
        std::fs::remove_dir_all(out)?;
    }

    info!("creating out directory");

    std::fs::create_dir(out)?;

    info!("generating HTML files");

    for site in sites_tree.iter() {
        let site_value = site.val();

        info!(
            "generating {}",
            site_value.file_name().map(|v| v.to_str()).flatten().ok_or(anyhow!(
                "unable to obtain file name from {:?}",
                site_value.canonicalize()?.display()
            ))?
        );

        let joined = out.join(site_value.strip_prefix("..").unwrap());

        if site_value.is_dir() {
            std::fs::create_dir_all(&joined).unwrap();

            let mut index_html = std::fs::File::create(joined.join("index.html"))?;

            index_html.write_all(meta_str(site.layer()).as_bytes())?;
            index_html.write_all(pre_header.as_bytes())?;
            index_html.write_all(header.as_bytes())?;
            index_html.write_all(post_header.as_bytes())?;
            index_html.write_all(pre_content.as_bytes())?;
            index_html.write_all(post_content.as_bytes())?;
        } else {
            bail!("non-directory found in file-tree at {}", site_value.canonicalize()?.display());
        }
    }

    Ok(())
}
