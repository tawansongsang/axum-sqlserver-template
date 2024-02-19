USE DbTemplate;

DROP PROCEDURE IF EXISTS dbo.sp_userinfo_create;

BEGIN
	EXEC ('
	CREATE PROCEDURE dbo.sp_userinfo_create
		@Username VARCHAR(51)
		, @Name NVARCHAR(251) = NULL
		, @Email VARCHAR(51)
		, @EmailVerified DATETIME2 = NULL
		, @Password VARCHAR(201)
		, @Role VARCHAR(5) = ''USER''
		, @CreateBy UNIQUEIDENTIFIER = NULL
		, @Active CHAR(1) = ''Y''
		, @Deleted CHAR(1) = ''N''
		, @DeleteOn DATETIME2 = NULL
		, @DeleteBy UNIQUEIDENTIFIER = NULL
	AS
	
	SET NOCOUNT ON;
	INSERT INTO dbo.UserInfo (Username, Name
                            , Email, EmailVerified
							, Password, Role
                            , CreateBy, CreateOn, UpdateBy, UpdateOn
							, Active, Deleted, DeleteOn, DeleteBy)
	OUTPUT INSERTED.UserInfoID
	VALUES (@Username, @Name, @Email
			, @EmailVerified, @Password, @Role
			, @CreateBy, GETDATE(), @CreateBy, GETDATE()
			, @Active, @Deleted, @DeleteOn, @DeleteBy);
')
END
