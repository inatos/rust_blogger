$(function () 
{
  // Auto resize textarea
  autosize($("textarea"));
});   

///
// Strips escaped chars from string.
///  
function DecodeHtml(input)
{
    var e = document.createElement("div");
    e.innerHTML = input;
    return e.childNodes.length === 0 ? "" : e.childNodes[0].nodeValue;
}

///
// Get cookie by name.
/// 
function GetCookie(cname) 
{
    var name = cname + "=";
    var decodedCookie = decodeURIComponent(document.cookie);
    var ca = decodedCookie.split(";");
    for (var i = 0; i < ca.length; i++) 
    {
      var c = ca[i];
      while (c.charAt(0) == " ") 
      {
        c = c.substring(1);
      }
      if (c.indexOf(name) == 0) 
      {
        return c.substring(name.length, c.length);
      }
    }
    return "";
  }