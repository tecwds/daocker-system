'use strict';

/** @type {import('sequelize-cli').Migration} */
module.exports = {
  async up(queryInterface, Sequelize) {
    await queryInterface.bulkInsert('Users', [{
      studentId: 'heying2204',
      password: '123456',
      name: '何颖',
      age: 10,
      gender: 20,
      college: '计算机学院',
      major: '软件工程',
      grade: '2204',
      email: 'heying@example.com',
      createdAt: new Date(),
      updatedAt: new Date()
    }, {
      studentId: 'chenkeyi2204',
      password: '123456',
      name: '陈可怡',
      age: 20,
      gender: 20,
      college: '计算机学院',
      major: '软件工程',
      grade: '2204',
      email: 'chenkeyi@example.com',
      createdAt: new Date(),
      updatedAt: new Date()
    }, {
      studentId: 'xinghao2204',
      password: '123456',
      name: '幸好',
      age: 20,
      gender: 20,
      college: '计算机学院',
      major: '软件工程',
      grade: '2204',
      email: 'xinghao@example.com',
      createdAt: new Date(),
      updatedAt: new Date()
    }, {
      studentId: 'panwenbao2204',
      password: '123456',
      name: '潘文宝',
      age: 20,
      gender: 10,
      college: '计算机学院',
      major: '软件工程',
      grade: '2204',
      email: 'panwenbao@example.com',
      createdAt: new Date(),
      updatedAt: new Date()
    }, {
      studentId: '2200770171',
      password: '123456',
      name: '罗广',
      age: 20,
      gender: 10,
      college: '计算机学院',
      major: '软件工程',
      grade: '2204',
      email: 'luoguang@example.com',
      createdAt: new Date(),
      updatedAt: new Date()
    }]);
  },

  async down(queryInterface, Sequelize) {
    await queryInterface.bulkDelete('Users', null, {});
  }
};
