use std::io::prelude::*;
use std::path::Path;

use anyhow::*;

pub fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() -> Result<()> {
    let content = Path::new("../content");

    if !content.exists() {
        bail!("invalid path to content: {}", std::fs::canonicalize(content)?.display());
    }

    println!("content path is {:?}", std::fs::canonicalize(content));

    if !content.is_dir() {
        bail!("content path isn't a directory");
    }

    let sites_folder = content.join("sites");

    if !sites_folder.exists() {
        bail!(
            "sites folder didn't exist in content: {}",
            std::fs::canonicalize(sites_folder)?.display()
        );
    }

    let meta_str = concat!(
        "<!DOCTYPE html>\n",
        "<html>\n",
        "<head>\n",
        "    <title>Rewrites Are Bliss</title>\n",
        "    <meta content=\"width=device-width, initial-scale=1.0\" name=\"viewport\">\n",
        "    <link rel=\"stylesheet\" type=\"text/css\" href=\"../../../styles/default.css\"/>\n",
        "    <link rel=\"stylesheet\" href=\"../../../styles/codetheme.css\">\n",
        "    <script src=\"https://kit.fontawesome.com/d7fa8be03e.js\" crossorigin=\"anonymous\"></script>\n",
        "    <script src=\"../../../vendor/highlight/highlight.min.js\"></script>\n",
        "    <script src=\"../../../scripts/main.js\"></script>\n",
        "</head>\n"
    );

    let pre_header = concat!(
        "<body>\n",
        "    <div id=\"page\">\n",
        "        <div id=\"header\">\n",
        "            <p id=\"header_title_label\">Rewrites Are Bliss</p>\n",
        "            <div id=\"header_container\">\n"
    );

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

    let sites = std::fs::read_dir(sites_folder)?
        .into_iter()
        .map(|v| v.map(|v| v.path()))
        .collect::<Result<Vec<_>, _>>()?;

    let main_sites_str = sites
        .iter()
        .map(|v| {
            Ok(format!(
                concat!(
                    "            <div class=\"tl_menu_item\">\n",
                    "               <p>{}</p>\n",
                    "            </div>\n"
                ),
                v.file_name()
                    .map(|v| v.to_str().map(|v| capitalise(v)))
                    .flatten()
                    .ok_or(anyhow!("cannot obtain filename for site {:?}", v))?
            ))
        })
        .collect::<Result<String, _>>()?;

    for path in sites {
        let mut index_html = std::fs::File::create(path.join("index.html"))?;
        index_html.write_all(meta_str.as_bytes())?;
        index_html.write_all(pre_header.as_bytes())?;
        index_html.write_all(main_sites_str.as_bytes())?;
        index_html.write_all(post_header.as_bytes())?;
        index_html.write_all(pre_content.as_bytes())?;
        index_html.write_all(post_content.as_bytes())?;
    }

    Ok(())
}
