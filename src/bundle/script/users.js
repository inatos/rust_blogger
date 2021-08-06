$(function ()   
{
    // Set scroll position
    sessionStorage.scrollTop = $(this).scrollTop();

    // Submit Comment Form w/ validation 
    $("#editUserForm").submit(function (event)
    {
        event.preventDefault();
        if (confirm("Save changes?"))
        {
            $.ajax({
                url: "/edit_user",
                type: "POST",
                cache: false,
                async: false,
                data: $(this).serialize(),
                success: function (result)
                {
                    alert(result + " has been updated!");
                    window.location.reload();
                },
                error: function (xhr, status, error)
                {
                    alert("Edit request could not be processed. \nStatus: " + status + "\n" + error + "\n" + xhr.responseText);
                }           
            }); 
        }
    });
});

// Save scroll location //
$(window).scroll(function() {
    sessionStorage.scrollTop = $(this).scrollTop();
});

///
// Populate modal with row vals.
///  
function PopulateModal(userId)
{
    $("#editUserId").val(userId);
    $("#editUsername").val($("#username_" + userId).text());
    $("#editEmail").val($("#email_" + userId).text());
    $("#statusDdl").val($("#status_" + userId).val());
}