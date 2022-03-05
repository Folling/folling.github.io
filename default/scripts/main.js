document.addEventListener("DOMContentLoaded", () => {
    let header = document.getElementById("header");
    let page = document.getElementById("page");
    let ticking = false;

    page.onscroll = (e) => {
        lastKnownScrollPosition = window.scrollY;

        if (!ticking) {
            window.requestAnimationFrame(function() {
                if (page.scrollTop > 0) {
                    header.classList.add("scrolled_page");
                } else {
                    header.classList.remove("scrolled_page");
                }
                
                ticking = false;
            });
  
            ticking = true;
        }
    };
});
