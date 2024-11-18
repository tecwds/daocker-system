// 统一返回结果的工具类
class Result {
  // 成功返回
  static success(res, data = null, message = 'success') {
    res.json({
      code: 200,
      message,
      data
    });
  }

  // 失败返回 
  static error(res, message = 'error', code = 500) {
    res.json({
      code,
      message,
      data: null
    });
  }

  // 参数错误返回
  static badRequest(res, message = '参数错误') {
    res.json({
      code: 400,
      message,
      data: null
    });
  }

  // 未授权返回
  static unauthorized(res, message = '未授权') {
    res.json({
      code: 401, 
      message,
      data: null
    });
  }
}

module.exports = Result;
