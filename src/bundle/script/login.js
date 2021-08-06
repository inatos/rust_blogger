$(document).ready(function ()   
{
    // Submit Login Form w/ post validation 
    $("#loginForm").submit(function (event)
    {
        event.preventDefault();

        $.ajax({
            url: "/authenticate",
            type: "POST",
            cache: false,
            async: false,
            data: $(this).serialize(),
            success: function (result)
            {
                alert(result);
                location.reload();
            },
            error: function (xhr, status, error)
            {
                alert("Login request could not be processed. \nStatus: " + status + "\n: " + error + "\n" + xhr.responseText);
            }           
        });     
    });
});