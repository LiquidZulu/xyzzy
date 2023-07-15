(() => {
  const waitForDOM = (cb) => {
    if (document.readyState !== "loading") {
      cb();
    } else {
      document.addEventListener("DOMContentLoaded", () => {
        cb();
      });
    }
  };

  // remove the annoying &pp= thing. Taken from: https://news.ycombinator.com/item?id=28005638
  /*const origFetch = window.fetch;
  window.fetch = async (url, init) => {
    if (url.url.startsWith("https://www.youtube.com/youtubei/v1/browse?key=")) {
      let response = await (await origFetch(url)).text();
      response = response.replace(/(\&pp\=.*?)",/g, '",');
      return new Response(response);
    } else {
      return origFetch(url);
    }
  };*/

  waitForDOM(() => {
    if (document.location.pathname == "/feed/subscriptions") {
      document.querySelectorAll("a[href*=\\&pp\\=]").forEach((el) => {
        el.href = decodeURIComponent(
          "/watch?v=" + el.href.match(/v=(.*?)(&|$)/)[1]
        );
      });
    }

    const anchors = document.getElementsByTagName("a");

    for (let i = 0; i < anchors.length; ++i) {
      if (!!anchors[i].href.match(/^.*watch.+(v=[A-Za-z0-9\-\_]{11}).*$/)) {
        anchors[i].href = anchors[i].href.replace(/((\&|\?)pp=.[^\&]+)/g, "");
      }
    }
  });

  // Remove gradient, it glitches out on fx
  waitForDOM(() => {
    try {
      document.querySelector(".ytp-gradient-bottom").style +=
        "visibility: hidden;";
    } catch (e) {}
  });
})();
