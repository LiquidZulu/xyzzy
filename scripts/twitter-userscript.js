(() => {
  const removeThatDamnedBox = () => {
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
  };

  setInterval(() => {
    if (window.location.href.split("/")[4] == "verified-get-verified")
      removeThatDamnedBox();
  }, 100);
})();
