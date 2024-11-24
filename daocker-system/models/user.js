'use strict';
const {
  Model
} = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  class User extends Model {
    /**
     * Helper method for defining associations.
     * This method is not a part of Sequelize lifecycle.
     * The `models/index` file will call this method automatically.
     */
    static associate(models) {
      // define association here
    }
  }
  User.init({
    studentId: DataTypes.STRING,
    password: DataTypes.STRING,
    name: DataTypes.STRING,
    age: DataTypes.INTEGER,
    gender: DataTypes.INTEGER,
    avatar: DataTypes.STRING,
    email: DataTypes.STRING,
    college: DataTypes.STRING,
    major: DataTypes.STRING,
    grade: DataTypes.STRING
  }, {
    sequelize,
    modelName: 'User',
  });
  return User;
};