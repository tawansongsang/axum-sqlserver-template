USE DbTemplate;

DROP PROCEDURE IF EXISTS dbo.sp_userinfo_create;

BEGIN
	EXEC ('
	CREATE PROCEDURE dbo.sp_userinfo_create 
		@Username VARCHAR(51) = NULL
		, @Name NVARCHAR(51) = NULL
		, @Email VARCHAR(51) = NULL
		, @Password VARCHAR(201) = NULL
		, @CreateBy UNIQUEIDENTIFIER = NULL
	AS
	SET NOCOUNT ON;
	INSERT INTO dbo.UserInfo (Username
							, Name
                            , Email
							, Password
                            , CreateBy
							, UpdateBy)
	OUTPUT Inserted.UserInfoID
	VALUES (@Username
			, @Name
			, @Email
			, @Password
			, @CreateBy
			, @CreateBy);
')
END