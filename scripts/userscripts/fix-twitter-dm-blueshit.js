// ==UserScript==
// @name         fix-twitter-dm-blueshit
// @namespace    Twitter
// @version      0.1
// @description  Remove subscribe to blue warning from twitter DMs
// @author       LiquidZulu
// @match        https://twitter.com/*
// @icon         https://www.google.com/s2/favicons?sz=64&domain=twitter.com
// @grant        none
// ==/UserScript==

(() => {
  const removeThatDamnedBox = () => {
    try {
      const spans = document.getElementsByTagName("span");
      let theSpan = null;

      for (let i = 0; i < spans.length && !theSpan; ++i) {
        if (
          spans[i].innerHTML.toLowerCase().startsWith("get verified to message")
        ) {
          theSpan = spans[i];
        }
      }

      theSpan.parentElement.parentElement.parentElement
        .querySelector('[role="button"]')
        .click();
    } catch (e) {
      console.error(e);
    } finally {
      console.log("Removed the blueshit");
    }
  };

  setInterval(() => {
    if (window.location.href.split("/")[4] == "verified-get-verified")
      removeThatDamnedBox();
  }, 100);
})();
