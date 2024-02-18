USE DbTemplate;

DROP PROCEDURE IF EXISTS dbo.sp_GetErrorInfo;

BEGIN
    EXEC ('
	CREATE PROCEDURE dbo.sp_GetErrorInfo  
    AS  
    SELECT   
        ERROR_NUMBER() AS ErrorNumber  
        ,ERROR_SEVERITY() AS ErrorSeverity  
        ,ERROR_STATE() AS ErrorState  
        ,ERROR_LINE () AS ErrorLine  
        ,ERROR_PROCEDURE() AS ErrorProcedure  
        ,ERROR_MESSAGE() AS ErrorMessage;  
')
END