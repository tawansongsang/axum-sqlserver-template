USE DbTemplate;

DROP PROCEDURE IF EXISTS dbo.sp_userinfo_delete;

BEGIN
    EXEC ('
	CREATE PROCEDURE dbo.sp_userinfo_delete
		@UserInfoID UNIQUEIDENTIFIER
		, @DeleteBy UNIQUEIDENTIFIER

	AS
	
	SET NOCOUNT ON;
	UPDATE dbo.UserInfo
	SET DeleteBy=@DeleteBy, Deleted=''Y'', DeleteOn=GETDATE()
    WHERE UserInfoID=@UserInfoID;
')
END
