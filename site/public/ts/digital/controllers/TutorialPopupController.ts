import $ from "jquery";

export class TutorialPopupController {
    public constructor() {
        $("#header-help-tour-button").click(function(){
            $('.hover_bkgr_fricc').show();
            $('.popupStep1').show();
            $('.popupStep2').hide();
            $('.popupStep3').hide();
         });
         $('.popupStep1').click(function(){
             $('.popupStep1').hide();
             $('.popupStep2').show();
         })
         $('.popupStep2').click(function(){
             $('.popupStep2').hide();
             $('.popupStep3').show();
         })
         $('.popupStep3').click(function(){
             $('.popupStep3').hide();
             $('.hover_bkgr_fricc').hide();
         })
         $('.popupCloseButton').click(function(){
             $('.hover_bkgr_fricc').hide();
         });
    }
}