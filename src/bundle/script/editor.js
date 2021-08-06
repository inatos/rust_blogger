$(function ()   
{
    // Submit Post Form w/ validation 
    $("#postForm").submit(function (event)
    {
        event.preventDefault();
        if (confirm("Are you ready to submit your work?"))
        {
            // Make sure we're in the right mode
            if ($("#docModeToggle").is(":checked"))
            {
                SetDocMode(false);
            }

            // Set content
            $("#postContent").val($("#postContentBox").html());

            // Create/Update by mode
            var mode = $("#modeDdlBtn").val();
            switch (mode)
            {
                case "create":
                    $.ajax({
                        url: "/create_post",
                        type: "POST",
                        cache: false,
                        async: false,
                        data: $(this).serialize(),
                        success: function (result)
                        {
                            if (confirm("Post created! Would you like to view it?"))
                            {
                                window.location.replace(result);
                            }
                            else
                            {
                                window.location.replace("/editor");
                            }
                        },
                        error: function (xhr, status, error)
                        {
                            alert("Create request could not be processed. \nStatus: " + status + "\n" + error + "\n" + xhr.responseText);
                        }           
                    });
                    break;

                case "delete":
                    $.ajax({
                        url: "/delete_post",
                        type: "POST",
                        cache: false,
                        async: false,
                        data: $(this).serialize(),
                        success: function (result)
                        {
                            alert("Post deleted!");
                            window.location.replace(result);
                        },
                        error: function (xhr, status, error)
                        {
                            alert("Delete request could not be processed. \nStatus: " + status + "\n" + error + "\n" + xhr.responseText);
                        }           
                    });
                    break;

                case "edit":
                    $.ajax({
                        url: "/edit_post",
                        type: "POST",
                        cache: false,
                        async: false,
                        data: $(this).serialize(),
                        success: function (result)
                        {
                            if (confirm("Post updated! Would you like to view it?")) 
                            {
                                window.location.replace(result);
                            }
                            else
                            {
                                window.location.replace("/editor");
                            }
                        },
                        error: function (xhr, status, error)
                        {
                            alert("Edit request could not be processed. \nStatus: " + status + "\n" + error + "\n" + xhr.responseText);
                        }           
                    });
                    break;

                default:
                    break;
            }
        }
    });

    // Mode listener
    $("[name='modeBtn']").on("click", function ()
    {
        $("#modeDdlBtn").text($(this).text());
        $("#modeDdlBtn").val($(this).val());

        // Default New Group option
        $("#newGroupDdlBtn").text("no");
        $("#groupName").val("");
        $("#groupName").attr("hidden", true);
        $("#groups").selectpicker("show");

        // Default mode specific fields
        switch ($(this).val())
        {
            case "create":
                $("#postCol").attr("hidden", true);
                $("#posts option:selected").remove();
                $("#posts").selectpicker("refresh");
                $("#groups option:selected").remove();
                $("#groups").selectpicker("refresh");
                $("#postId").val("");
                $("#groupId").val("");
                $("#group").val("");
                $("#title").val("");              
                break;

            case "delete":
            case "edit":
                // Display posts
                $("#postCol").removeAttr("hidden");
                break;

            default:
                break;
        }
    });

    // Post listener
    $("#posts").on("change", function ()
    {
        var postId = $(this).val();
        var mode = $("#modeDdlBtn").val();

        // Load post data
        if (postId.length > 0)
        {
            window.location.replace("/editor?post_id=" + postId + "&mode=" + mode);
        }
    });

    // Enable live search Post dropdown
    $(".select-post").selectpicker(
    {
        liveSearch: true,
        showSubtext: true,
        title: "Select a post to edit...",
        dropupAuto: false
    });
    $(".select-post").selectpicker("setStyle", "ddl-custom", "add");

    // Enable live search Group dropdown
    $(".select-group").selectpicker(
    {
        liveSearch: true,
        showSubtext: true,
        title: "Select a group...",
        dropupAuto: false
    });
    $(".select-group").selectpicker("setStyle", "ddl-custom", "add");

    // Category listener
    $("[name='categoryBtn']").on("click", function ()
    {
        $("#categoryDdlBtn").text($(this).text());
        $("#postCategoryId").val($(this).val());
    });

    // Visibility listener
    $("[name='visibilityBtn']").on("click", function ()
    {
        $("#visibilityDdlBtn").text($(this).text());
        $("#postPublished").val($(this).val());
    });

    // Embedded listener
    $("[name='embeddedBtn']").on("click", function ()
    {
        $("#embeddedDdlBtn").text($(this).text());
        $("#postEmbedded").val($(this).val());
    });

    // New Group listener
    $("[name='newGroupBtn']").on("click", function ()
    {
        $("#newGroupDdlBtn").text($(this).text());
        $("#groupName").val("");

        // Toggle select/input
        if ($(this).val())
        {
            $("#groups").selectpicker("hide");
            $("#groupName").removeAttr("hidden");
        }
        else 
        {
            $("#groupName").attr("hidden", true);
            $("#groups").selectpicker("show");
        } 
    });

    // Format listener
    $("[name='formatBtn']").on("click", function ()
    {
        FormatDoc("formatblock", $(this).val());
        $("#formatDdlBtn").text($(this).text());
    });

    // Font listener
    $("[name='fontBtn']").on("click", function ()
    {
        FormatDoc("fontname", $(this).val());
        $("#fontDdlBtn").text($(this).text());
    });
    
    // Font Size listener
    $("[name='fontSizeBtn']").on("click", function ()
    {
        FormatDoc("fontsize", $(this).val());
        $("#fontSizeDdlBtn").text($(this).text());
    });

    // Color listener
    $("[name='colorBtn']").on("click", function ()
    {
        FormatDoc("forecolor", $(this).val());
        $("#colorDdlBtn").text($(this).text());
    });

    // Background listener
    $("[name='backgroundBtn']").on("click", function ()
    {
        FormatDoc("backcolor", $(this).val());
        $("#backgroundDdlBtn").text($(this).text());
    });

    // Clean listener
    $("#cleanBtn").on("click", function ()
    {
        if (ValidateMode() && confirm("Clean HTML?"))
        {
            oDoc.innerHTML = sDefTxt
        };
    });

    // Print listener
    $("#printBtn").on("click", function ()
    {
        PrintDoc();
    });

    // Quote listener
    $("#quoteBtn").on("click", function ()
    {
        FormatDoc("formatblock", "blockquote");
    });

    // Hyperlink listener
    $("#hyperlinkBtn").on("click", function ()
    {
        var sLnk = prompt("Write the URL here", "http:\/\/");
        if (sLnk && sLnk != "" && sLnk != "http://")
        {
            FormatDoc("createlink", sLnk)
        }
    });

    // Icon listener
    $("[name='iconBtn']").on("click", function ()
    {
        FormatDoc($(this).prop("alt"));
    });


    // Doc Mode listener
    $("#docModeToggle").on("change", function ()
    {
        SetDocMode($(this).is(":checked"));
    });

    // Post Content Box
    $("#postContentBox").on("blur", function ()
    {
        $(this).text($(this).html());
        $(this).html($(this).text());
    });
    
  
    var oDoc, sDefTxt;
    function InitiateDoc() 
    {
        oDoc = document.getElementById("postContentBox");
        sDefTxt = oDoc.innerHTML;
        if ($("#docModeToggle").is(":checked")) 
        { 
            SetDocMode(true); 
        }
    }

    function FormatDoc(sCmd, sValue) 
    {
        if (ValidateMode()) 
        { 
            document.execCommand(sCmd, false, sValue); 
            oDoc.focus(); 
        }
    }

    function ValidateMode() 
    {
        if (!$("#docModeToggle").is(":checked")) 
        { 
            return true; 
        }
        alert("Uncheck \"Show HTML\".");
        oDoc.focus();
        return false;
    }

    function SetDocMode(bToSource) 
    {
        var oContent;
        if (bToSource) 
        {
            oContent = document.createTextNode(oDoc.innerHTML);
            oDoc.innerHTML = "";
            var oPre = document.createElement("pre");
            oDoc.contentEditable = false;
            oPre.id = "sourceText";
            oPre.contentEditable = true;
            oPre.appendChild(oContent);
            oDoc.appendChild(oPre);
            document.execCommand("defaultParagraphSeparator", false, "div");
        } 
        else 
        {
            if (document.all) 
            {
                oDoc.innerHTML = oDoc.innerText;
            } 
            else 
            {
                oContent = document.createRange();
                oContent.selectNodeContents(oDoc.firstChild);
                oDoc.innerHTML = oContent.toString();
            }
            oDoc.contentEditable = true;
        }
        oDoc.focus();
    }

    function PrintDoc() 
    {
        if (!ValidateMode()) { return; }

        var oPrntWin = window.open("","_blank","width=450,height=470,left=400,top=100,menubar=yes,toolbar=no,location=no,scrollbars=yes");
        oPrntWin.document.open();
        oPrntWin.document.write("<!doctype html><html><head><title>Print<\/title><\/head><body onload=\"print();\">" + oDoc.innerHTML + "<\/body><\/html>");
        oPrntWin.document.close();
    }

    InitiateDoc();
    $("#postContentBox").html($("#postContentBox").text());
});