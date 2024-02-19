USE DbTemplate;

DROP PROCEDURE IF EXISTS dbo.sp_userinfo_get;

BEGIN
    EXEC ('
	CREATE PROCEDURE dbo.sp_userinfo_get
		@start INT = 0
		, @limit INT = 100
		, @order_by VARCHAR(100) = ''CreateOn''
		, @order_type VARCHAR(4) = ''ASC''
	AS
	
	SET NOCOUNT ON;
	SELECT * FROM UserInfo ui
    ORDER BY 
        CASE WHEN @order_by = ''CreateOn'' AND @order_type =''ASC'' THEN CreateOn END ,
        CASE WHEN @order_by = ''CreateOn'' AND @order_type =''DESC'' THEN CreateOn END DESC,
        CASE WHEN @order_by = ''CreateBy'' AND @order_type =''ASC'' THEN CreateBy END ,
        CASE WHEN @order_by = ''CreateBy'' AND @order_type =''DESC'' THEN CreateBy END DESC,
        CASE WHEN @order_by = ''UpdateOn'' AND @order_type =''ASC'' THEN UpdateOn END ,
        CASE WHEN @order_by = ''UpdateOn'' AND @order_type =''DESC'' THEN UpdateOn END DESC,
        CASE WHEN @order_by = ''UpdateBy'' AND @order_type =''ASC'' THEN UpdateBy END ,
        CASE WHEN @order_by = ''UpdateBy'' AND @order_type =''DESC'' THEN UpdateBy END DESC,
        CASE WHEN @order_by = ''DeleteOn'' AND @order_type =''ASC'' THEN DeleteOn END ,
        CASE WHEN @order_by = ''DeleteOn'' AND @order_type =''DESC'' THEN DeleteOn END DESC,
        CASE WHEN @order_by = ''DeleteBy'' AND @order_type =''ASC'' THEN DeleteBy END ,
        CASE WHEN @order_by = ''DeleteBy'' AND @order_type =''DESC'' THEN DeleteBy END DESC,
    OFFSET @start ROWS 
    FETCH NEXT @limit ROWS ONLY
')
END