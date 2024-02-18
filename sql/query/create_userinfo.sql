-- USE DbTemplate;
-- INSERT INTO dbo.UserInfo
--     (Username, [Name], Email, EmailVerified, [Password])
-- VALUES
--     ('demo1@demo.com', 'demo', 'demo1@demo.com', GETDATE(), 'demo1') 

USE DbTemplate;
EXEC dbo.sp_userinfo_create
    @Username = 'demo2@demo.com',
    @Name = N'demo2',
    @Email = 'demo2@demo.com',
    @Password = 'demo2',
    @CreateBy = NULL;