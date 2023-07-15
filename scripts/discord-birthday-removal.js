(() => {
  const divs = document.getElementsByTagName("div");

  for (let i = 0; i < divs.length; ++i) {
    if (divs[i].innerHTML == "Discord’s Birthday") {
      console.log("Found one!");
      console.log(divs[i]);
      divs[
        i
      ].parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.style.display =
        "none";

      // I assume there aren't multiple of these, so terminate the loop
      break;
    }
  }
})();
