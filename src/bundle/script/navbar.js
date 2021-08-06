$(document).ready(function ()   
{
    // Logout 
    $("#logoutBtn").click(function ()
    {
        $.ajax({
            url: "/logout",
            type: "POST",
            cache: false,
            async: false,
            success: function (result)
            {
                alert(result);
                location.reload();
            },
            error: function (xhr, status, error)
            {
                alert("Logout request could not be processed. \nStatus: " + status + "\n " + error + "\n" + xhr.responseText);
            }           
        });     
    });
});