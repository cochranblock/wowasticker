// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
package org.cochranblock.wowasticker;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.JavascriptInterface;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;

/**
 * f149=MainActivity. WebView host for sticker chart UI.
 * Rust lib (libwowasticker.so) provides the data layer via JNI bridge.
 * HTML/JS UI embedded as a string — no assets directory needed.
 */
public class MainActivity extends Activity {

    static {
        System.loadLibrary("wowasticker");
    }

    // JNI bridge to Rust
    private static native String jniRunDemo();
    private static native String jniGetReport(String date);
    private static native String jniSetSticker(long blockId, int value);
    private static native String jniGetBlocks();
    private static native String jniGetProgress();

    private WebView webView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        webView = findViewById(R.id.webview);
        WebSettings settings = webView.getSettings();
        settings.setJavaScriptEnabled(true);
        settings.setDomStorageEnabled(true);

        webView.addJavascriptInterface(new StickerBridge(), "WowaSticker");
        webView.setWebViewClient(new WebViewClient());

        // Load embedded HTML UI
        webView.loadDataWithBaseURL(null, buildHtml(), "text/html", "UTF-8", null);
    }

    /**
     * JS bridge: called from WebView JavaScript.
     */
    class StickerBridge {
        @JavascriptInterface
        public String getBlocks() {
            try { return jniGetBlocks(); }
            catch (UnsatisfiedLinkError e) { return "[]"; }
        }

        @JavascriptInterface
        public String setSticker(long blockId, int value) {
            try { return jniSetSticker(blockId, value); }
            catch (UnsatisfiedLinkError e) { return "{\"error\":\"jni\"}"; }
        }

        @JavascriptInterface
        public String getReport(String date) {
            try { return jniGetReport(date); }
            catch (UnsatisfiedLinkError e) { return "Report unavailable"; }
        }

        @JavascriptInterface
        public String getProgress() {
            try { return jniGetProgress(); }
            catch (UnsatisfiedLinkError e) { return "0/15"; }
        }
    }

    private String buildHtml() {
        return "<!DOCTYPE html>\n"
            + "<html><head><meta charset='utf-8'>\n"
            + "<meta name='viewport' content='width=device-width, initial-scale=1, user-scalable=no'>\n"
            + "<style>\n"
            + "* { box-sizing: border-box; margin: 0; padding: 0; }\n"
            + "body { font-family: system-ui, sans-serif; padding: 20px; padding-bottom: 120px; background: #fafafa; }\n"
            + "h1 { font-size: 1.5rem; margin-bottom: 4px; }\n"
            + "#progress { font-size: 1rem; color: #666; margin-bottom: 16px; }\n"
            + ".card { padding: 12px 15px; margin-bottom: 8px; border-radius: 8px; background: #f0f0f0; border: 2px solid transparent; }\n"
            + ".card.selected { background: #e3f2fd; border-color: #007AFF; }\n"
            + ".card-header { display: flex; justify-content: space-between; align-items: center; font-weight: 600; }\n"
            + ".score-btns { display: flex; gap: 8px; margin-top: 8px; }\n"
            + ".score-btns button { flex: 1; padding: 10px; border-radius: 6px; border: 1px solid #ccc; background: #fff; font-size: 1rem; cursor: pointer; min-height: 44px; }\n"
            + ".score-btns button.active-0 { background: #ffcdd2; }\n"
            + ".score-btns button.active-1 { background: #fff9c4; }\n"
            + ".score-btns button.active-2 { background: #c8e6c9; }\n"
            + "#bottom { position: fixed; bottom: 0; left: 0; right: 0; padding: 16px 20px; background: #fff; border-top: 1px solid #ccc; }\n"
            + "#status { font-size: 0.85rem; color: #666; margin-bottom: 8px; }\n"
            + "#share-btn { width: 100%; padding: 14px; font-size: 1rem; background: #e8f5e9; color: #2e7d32; border-radius: 10px; border: 1px solid #a5d6a7; cursor: pointer; }\n"
            + "</style></head><body>\n"
            + "<h1>Luka's Sticker Chart</h1>\n"
            + "<div id='progress'>Loading...</div>\n"
            + "<div id='blocks'></div>\n"
            + "<div id='bottom'>\n"
            + "  <div id='status'>Tap a block, then score.</div>\n"
            + "  <button id='share-btn' onclick='shareReport()'>Share Daily Report</button>\n"
            + "</div>\n"
            + "<script>\n"
            + "let selected = -1;\n"
            + "let blocks = [];\n"
            + "function refresh() {\n"
            + "  try { blocks = JSON.parse(WowaSticker.getBlocks()); } catch(e) { blocks = []; }\n"
            + "  let html = '';\n"
            + "  blocks.forEach((b, i) => {\n"
            + "    let sel = i === selected ? 'selected' : '';\n"
            + "    let sticker = b.value === 2 ? '●●' : b.value === 1 ? '●' : '○';\n"
            + "    html += '<div class=\"card ' + sel + '\" onclick=\"selectBlock(' + i + ')\">';\n"
            + "    html += '<div class=\"card-header\"><span>' + b.name + '</span><span>' + sticker + '</span></div>';\n"
            + "    if (i === selected) {\n"
            + "      html += '<div class=\"score-btns\">';\n"
            + "      for (let v = 0; v <= 2; v++) {\n"
            + "        let cls = b.value === v ? 'active-' + v : '';\n"
            + "        let label = v === 0 ? '0' : v === 1 ? '1' : '2';\n"
            + "        html += '<button class=\"' + cls + '\" onclick=\"score(' + b.id + ',' + v + '); event.stopPropagation();\">' + label + '</button>';\n"
            + "      }\n"
            + "      html += '</div>';\n"
            + "    }\n"
            + "    if (b.note) html += '<div style=\"font-size:0.8rem;color:#555;margin-top:4px;font-style:italic\">\"' + b.note + '\"</div>';\n"
            + "    html += '</div>';\n"
            + "  });\n"
            + "  document.getElementById('blocks').innerHTML = html;\n"
            + "  try { document.getElementById('progress').textContent = WowaSticker.getProgress(); } catch(e) {}\n"
            + "}\n"
            + "function selectBlock(i) { selected = i; refresh(); }\n"
            + "function score(blockId, value) {\n"
            + "  try { WowaSticker.setSticker(blockId, value); } catch(e) {}\n"
            + "  document.getElementById('status').textContent = blocks[selected].name + ': saved!';\n"
            + "  refresh();\n"
            + "}\n"
            + "function shareReport() {\n"
            + "  let d = new Date().toISOString().slice(0,10);\n"
            + "  try {\n"
            + "    let report = WowaSticker.getReport(d);\n"
            + "    if (navigator.clipboard) navigator.clipboard.writeText(report);\n"
            + "    document.getElementById('status').textContent = 'Report copied!';\n"
            + "  } catch(e) { document.getElementById('status').textContent = 'Share failed'; }\n"
            + "}\n"
            + "refresh();\n"
            + "</script></body></html>";\n"
            ;
    }
}
