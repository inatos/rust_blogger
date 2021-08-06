$(function ()   
{
    // Set scroll position
    sessionStorage.scrollTop = $(this).scrollTop();

    // Submit Comment Form w/ validation 
    $("form[name='comment_form']").submit(function (event)
    {
        event.preventDefault();
        if (confirm("Post comment?"))
        {
            $.ajax({
                url: "/post_comment",
                type: "POST",
                cache: false,
                async: false,
                data: $(this).serialize(),
                success: function (result)
                {
                    alert(result);
                    window.location.reload();
                },
                error: function (xhr, status, error)
                {
                    alert("Create request could not be processed. \nStatus: " + status + "\n" + error + "\n" + xhr.responseText);
                }           
            }); 
        }
    });
});

// Save scroll location //
$(window).scroll(function() {
    sessionStorage.scrollTop = $(this).scrollTop();
});