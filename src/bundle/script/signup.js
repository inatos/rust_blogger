$(document).ready(function ()   
{
    // Submit Signup Form w/ post validation 
    $("#signupForm").submit(function (event)
    {
        event.preventDefault();

        $.ajax({
            url: "/register_user",
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
                alert("Signup request could not be processed. \nStatus: " + status + "\n " + error + "\n" + xhr.responseText);
            }           
        });     
    });
});