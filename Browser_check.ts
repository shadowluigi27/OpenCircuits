function warning_message(hide_message: boolean): void{
    if(hide_message){
        document.getElementById("Browser-Warning-Message").style.visibility='hidden';
    }
    else{
        document.getElementById("Browser-Warning-Message").style.visibility='visible';
    }
}

export function isValidBrowser(): boolean {    
    var userAgent = navigator.userAgent;
    var Is_IExplorerAgent = userAgent.indexOf("MSIE") > -1 || userAgent.indexOf("rv:") > -1; 
    if(Is_IExplorerAgent)
    {
        warning_message(false);
        return false;
    }
    var index;
    var browser_version;
    if(index = userAgent.indexOf("Chrome") > -1)
    {
        browser_version = userAgent.substring(index+7);
        if(parseFloat(browser_version) > 86)
        {
            warning_message(false);
            return false;
        }
        warning_message(true);
        return true;
    }
    if(index = userAgent.indexOf("Firefox") > -1)
    {
        browser_version = userAgent.substring(index+8);
        if(parseFloat(browser_version) > 82)
        {
            warning_message(false);
            return false;
        }
        warning_message(true);
        return true;
    }
    if(index = userAgent.indexOf("Safari") > 10)
    {
        browser_version = userAgent.substring(index+8);
        if(parseFloat(browser_version) > 13)
        {
            warning_message(false);
            return false;
        }
        warning_message(true);
        return true;
    }
    if(index = userAgent.indexOf("OP") > -1)
    {
        index =  userAgent.indexOf("version");
        browser_version = userAgent.substring(index+8);
        if(parseFloat(browser_version) >72)
        {
            warning_message(false);
            return false;
        }
        warning_message(true);
        return true;  
    }
    warning_message(false);
    return true;
}





