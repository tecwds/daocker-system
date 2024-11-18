var express = require('express');
var router = express.Router();
const { User } = require('../models');
const Result = require('../utils/result');
const jwt = require('jsonwebtoken');

// 密钥（用于签名 JWT）
const SECRET_KEY = "lggbond";


// 登录路由
router.post('/login', async (req, res) => {
  const { studentId, password } = req.body;

  // 验证用户是否存在
  const user = await User.findOne({ where: { studentId } });
  if (!user) {
    return Result.error(res, "用户不存在");
  }

  // 验证密码
  const isPasswordValid = password == user.password;
  // console.log(user);

  if (!isPasswordValid) {
    return Result.error(res, "密码错误");
  }

  // 生成 JWT
  const token = jwt.sign({ id: user.id, studentId: user.studentId }, SECRET_KEY, {
    expiresIn: "1h", // 设置过期时间为 1 小时
  });

  Result.success(res, { token });
});

// 查询所有成员
router.get('/', async (req, res) => {
  const token = req.headers["authorization"];
  if (!token) {
    return Result.error(res, "请先登录");
  }
  // 验证 JWT
  jwt.verify(token, SECRET_KEY, async (err, decoded) => {
    if (err) {
      return Result.error(res, "请先登录");
    }

    const users = await User.findAll();
    Result.success(res, users);
  });

});

// 修改成员信息
router.put('/:id', async (req, res) => {
  const token = req.headers["authorization"];
  if (!token) {
    return Result.error(res, "请先登录");
  }

  jwt.verify(token, SECRET_KEY, async (err, decoded) => {
    if (err) {
      return Result.error(res, "请先登录");
    }

    try {
      const id = req.params.id;
      const { name, studentId, gender, email, avatar, password, age, grade, college, major, } = req.body;
      const user = await User.findByPk(id);

      if (!user) {
        return Result.error(res, "用户不存在");
      }
      console.log(name, studentId, gender, email, avatar, password, age, grade, college, major);

      await user.update({ name, studentId, gender, email, avatar, password, age, grade, college, major, updatedAt: new Date() });
      Result.success(res, user);
    } catch (error) {
      Result.error(res, "更新失败");
    }
  });
});

// 删除成员
router.delete('/:id', async (req, res) => {
  const token = req.headers["authorization"];
  if (!token) {
    return Result.error(res, "请先登录");
  }

  jwt.verify(token, SECRET_KEY, async (err, decoded) => {
    if (err) {
      return Result.error(res, "请先登录");
    }

    try {
      const id = req.params.id;
      const user = await User.findByPk(id);

      if (!user) {
        return Result.error(res, "用户不存在");
      }

      await user.destroy();
      Result.success(res, "删除成功");
    } catch (error) {
      Result.error(res, "删除失败");
    }
  });
});

// 新增成员
router.post('/', async (req, res) => {
  const token = req.headers["authorization"];
  if (!token) {
    return Result.error(res, "请先登录");
  }

  jwt.verify(token, SECRET_KEY, async (err, decoded) => {
    if (err) {
      return Result.error(res, "请先登录");
    }

    try {
      const userData = req.body;
      const user = await User.create({ ...userData, createdAt: new Date(), updatedAt: new Date() });
      Result.success(res, user);
    } catch (error) {
      Result.error(res, "创建失败");
    }
  });
});

module.exports = router;
