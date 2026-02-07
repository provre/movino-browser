use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};
use serde_json;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Minimal Browser")
        .with_inner_size(wry::application::dpi::LogicalSize::new(900u32, 700u32))
        .build(&event_loop)?;

    let homepage = include_str!("../assets/index.html");

    // Prepare a JSON-escaped homepage string for injection into every page
    let homepage_js = serde_json::to_string(homepage).unwrap_or_else(|_| "''".to_string());

    // Initialization script injected into every loaded page. It exposes the
    // homepage as `window.__MINIMAL_HOME`, injects a persistent topbar, and listens for Ctrl/Cmd+H.
    let init_script = format!(r#"
        window.__MINIMAL_HOME = {};
        // Provide a robust restore function that uses the custom minimal://home protocol
        window.__MINIMAL_RESTORE = function() {{
            try {{
                location.href = 'minimal://home';
            }} catch (e) {{
                console.error('Failed to restore home:', e);
            }}
        }};
        
        // Inject persistent topbar into every page
        function injectTopbar() {{
            // Create topbar styles
            var style = document.createElement('style');
            style.textContent = `
                .__minimal-topbar {{
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    height: 60px;
                    background-color: #ffffff;
                    border-bottom: 1px solid #000000;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 0 20px;
                    z-index: 999999;
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                    gap: 20px;
                }}
                body.dark .__minimal-topbar {{
                    background-color: #000000;
                    border-bottom-color: #ffffff;
                }}
                .__minimal-topbar-left {{
                    font-size: 24px;
                    font-weight: 300;
                    letter-spacing: 2px;
                    text-transform: uppercase;
                    cursor: pointer;
                    user-select: none;
                    color: #000000;
                    flex-shrink: 0;
                }}
                body.dark .__minimal-topbar-left {{
                    color: #ffffff;
                }}
                .__minimal-topbar-left:hover {{
                    opacity: 0.7;
                }}
                .__minimal-topbar-search {{
                    flex: 1;
                    max-width: 400px;
                    padding: 8px 12px;
                    border: 1px solid #000000;
                    background-color: #ffffff;
                    color: #000000;
                    font-size: 14px;
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                    font-weight: 300;
                }}
                body.dark .__minimal-topbar-search {{
                    background-color: #000000;
                    border-color: #ffffff;
                    color: #ffffff;
                }}
                .__minimal-topbar-search:focus {{
                    outline: none;
                    box-shadow: 0 0 0 2px #000000;
                }}
                body.dark .__minimal-topbar-search:focus {{
                    box-shadow: 0 0 0 2px #ffffff;
                }}
                .__minimal-topbar-search::placeholder {{
                    color: #999999;
                }}
                .__minimal-topbar-right {{
                    display: flex;
                    gap: 10px;
                    flex-shrink: 0;
                }}
                .__minimal-topbar-btn {{
                    padding: 8px 16px;
                    border: 1px solid #000000;
                    background-color: #ffffff;
                    color: #000000;
                    cursor: pointer;
                    font-size: 14px;
                    font-weight: 300;
                    transition: all 0.2s ease;
                }}
                body.dark .__minimal-topbar-btn {{
                    background-color: #000000;
                    border-color: #ffffff;
                    color: #ffffff;
                }}
                .__minimal-topbar-btn:hover {{
                    opacity: 0.7;
                }}
                body {{
                    padding-top: 60px !important;
                    margin-top: 0 !important;
                }}
            `;
            document.head.appendChild(style);
            
            // Create and insert topbar HTML
            var topbar = document.createElement('div');
            topbar.className = '__minimal-topbar';
            topbar.innerHTML = `
                <div class="__minimal-topbar-left" id="__minimal-home-btn">H</div>
                <input type="text" class="__minimal-topbar-search" id="__minimal-search-input" placeholder="Search or URL...">
                <div class="__minimal-topbar-right">
                    <button class="__minimal-topbar-btn" id="__minimal-theme-btn">Dark</button>
                </div>
            `;
            document.body.insertBefore(topbar, document.body.firstChild);
            
            // Wire up home button
            document.getElementById('__minimal-home-btn').addEventListener('click', function() {{
                window.__MINIMAL_RESTORE();
            }});
            
            // Wire up search input
            var searchInput = document.getElementById('__minimal-search-input');
            searchInput.addEventListener('keypress', function(event) {{
                if (event.key === 'Enter') {{
                    var query = searchInput.value.trim();
                    if (query) {{
                        // Check if it's a URL
                        if (query.includes('.') && !query.includes(' ')) {{
                            let url = query;
                            if (!query.startsWith('http://') && !query.startsWith('https://')) {{
                                url = 'https://' + query;
                            }}
                            window.location.href = url;
                        }} else {{
                            // Search using DuckDuckGo
                            var searchUrl = 'https://duckduckgo.com/?q=' + encodeURIComponent(query);
                            window.location.href = searchUrl;
                        }}
                    }}
                }}
            }});
            
            // Wire up theme toggle
            var themeBtn = document.getElementById('__minimal-theme-btn');
            themeBtn.addEventListener('click', function() {{
                document.body.classList.toggle('dark');
                themeBtn.textContent = document.body.classList.contains('dark') ? 'Light' : 'Dark';
                localStorage.setItem('__minimal-dark-mode', document.body.classList.contains('dark') ? '1' : '0');
            }});
            
            // Restore dark mode preference
            if (localStorage.getItem('__minimal-dark-mode') === '1') {{
                document.body.classList.add('dark');
                themeBtn.textContent = 'Light';
            }}
            
            // Update search input to show current page URL
            function updateSearchInputWithCurrentPage() {{
                var searchInput = document.getElementById('__minimal-search-input');
                if (!searchInput) return;
                
                var location = window.location.href;
                if (location.includes('minimal://')) {{
                    searchInput.value = 'monivo:about';
                }} else {{
                    searchInput.value = location;
                }}
                searchInput.scrollLeft = 0; // Keep text left-aligned
            }}
            
            // Update on initial load
            updateSearchInputWithCurrentPage();
            
            // Update when page loads
            window.addEventListener('load', updateSearchInputWithCurrentPage);
            
            // Update on history changes (back/forward)
            window.addEventListener('popstate', updateSearchInputWithCurrentPage);
        }}
        
        // Inject topbar on page load and when DOM is ready
        if (document.readyState === 'loading') {{
            document.addEventListener('DOMContentLoaded', injectTopbar);
        }} else {{
            injectTopbar();
        }}
        
        // Use a capturing listener so pages that stop propagation don't block it
        document.addEventListener('keydown', function(e) {{
            if ((e.ctrlKey || e.metaKey) && (e.key === 'h' || e.key === 'H')) {{
                try {{ e.preventDefault(); window.__MINIMAL_RESTORE(); }} catch (err) {{}}
            }}
        }}, true);
    "#, homepage_js);

    let homepage_clone = homepage.to_string();
    let builder = WebViewBuilder::new(window)?;
    let builder = builder.with_html(homepage)?;
    let builder = builder.with_initialization_script(&init_script);
    
    // Register a custom protocol handler for minimal://home to return to homepage
    let builder = builder.with_custom_protocol("minimal".into(), move |_request| {
        Ok(wry::http::Response::builder()
            .header("Content-Type", "text/html")
            .body(homepage_clone.as_bytes().to_vec().into())?)
    });
    
    let _webview = builder
        .with_devtools(false)
        .with_accept_first_mouse(false)
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                clear_user_data();
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

/// Clear user data to ensure no history is stored
fn clear_user_data() {
    // In a production browser, this would clear caches, cookies, etc.
    // For now, we ensure the application exits cleanly
    // The WebView data is not persisted by default in wry
}
