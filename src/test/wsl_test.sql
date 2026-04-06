-- ----------------------------------------
-- 1. 创建数据库
-- 建议使用 utf8mb4 字符集，支持更多字符（如表情符号）
-- ----------------------------------------
CREATE DATABASE IF NOT EXISTS `mini_ecommerce` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 切换到新创建的数据库
USE `mini_ecommerce`;

-- ----------------------------------------
-- 2. 商品分类表 (Category)
-- 包含自关联，支持多级分类
-- ----------------------------------------
CREATE TABLE `Category` (
    category_id INT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '分类ID',
    parent_id INT UNSIGNED NULL COMMENT '上级分类ID，NULL表示一级分类',
    name VARCHAR(100) NOT NULL COMMENT '分类名称',
    
    PRIMARY KEY (category_id),
    -- 自关联外键
    FOREIGN KEY (parent_id) REFERENCES `Category`(category_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品分类表';

-- ----------------------------------------
-- 3. 用户表 (User)
-- 存储用户基本信息和密码哈希
-- ----------------------------------------
CREATE TABLE `User` (
    user_id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户ID',
    username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名/登录账号',
    password_hash CHAR(60) NOT NULL COMMENT '密码哈希值（推荐使用bcrypt，长度约60）',
    email VARCHAR(100) NULL UNIQUE COMMENT '邮箱',
    phone VARCHAR(20) NULL UNIQUE COMMENT '手机号码',
    registration_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '注册时间',
    last_login DATETIME NULL COMMENT '最后登录时间',
    
    PRIMARY KEY (user_id),
    INDEX idx_phone (phone)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户表';

-- ----------------------------------------
-- 4. 用户地址表 (Address)
-- 用户可以有多个收货地址
-- ----------------------------------------
CREATE TABLE `Address` (
    address_id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '地址ID',
    user_id BIGINT UNSIGNED NOT NULL COMMENT '所属用户ID',
    recipient_name VARCHAR(50) NOT NULL COMMENT '收件人姓名',
    phone VARCHAR(20) NOT NULL COMMENT '联系电话',
    province VARCHAR(50) NOT NULL COMMENT '省份',
    city VARCHAR(50) NOT NULL COMMENT '城市',
    detail_address VARCHAR(255) NOT NULL COMMENT '详细地址',
    is_default TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否为默认地址 (0否, 1是)',
    
    PRIMARY KEY (address_id),
    INDEX idx_user_id (user_id),
    -- 外键关联到 User 表
    FOREIGN KEY (user_id) REFERENCES `User`(user_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户地址表';

-- ----------------------------------------
-- 5. 商品表 (Product)
-- 存储商品的基本信息和库存
-- ----------------------------------------
CREATE TABLE `Product` (
    product_id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '商品ID',
    category_id INT UNSIGNED NOT NULL COMMENT '所属分类ID',
    name VARCHAR(255) NOT NULL COMMENT '商品名称',
    description TEXT NULL COMMENT '商品详细描述',
    price DECIMAL(10, 2) NOT NULL COMMENT '销售价格',
    stock_quantity INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '库存数量',
    is_active TINYINT(1) NOT NULL DEFAULT 1 COMMENT '是否上架 (0下架, 1上架)',
    
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后更新时间',
    
    PRIMARY KEY (product_id),
    -- 索引和外键
    INDEX idx_category_id (category_id),
    -- 建议为名称创建索引以支持搜索
    FULLTEXT KEY ft_name_description (name, description), 
    
    FOREIGN KEY (category_id) REFERENCES `Category`(category_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品表';

-- ----------------------------------------
-- 6. 订单表 (Order)
-- 记录用户下单的核心信息
-- ----------------------------------------
CREATE TABLE `Order` (
    order_id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '订单ID',
    user_id BIGINT UNSIGNED NOT NULL COMMENT '下单用户ID',
    address_id BIGINT UNSIGNED NOT NULL COMMENT '收货地址ID快照',
    order_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '下单时间',
    total_amount DECIMAL(10, 2) NOT NULL COMMENT '订单总金额',
    -- 订单状态建议使用 ENUM 或 TINYINT，这里使用 ENUM
    status ENUM('待付款', '待发货', '待收货', '已完成', '已取消', '退款中') NOT NULL DEFAULT '待付款' COMMENT '订单状态',
    
    payment_method VARCHAR(50) NULL COMMENT '支付方式',
    payment_time DATETIME NULL COMMENT '支付时间',
    shipping_fee DECIMAL(10, 2) NOT NULL DEFAULT 0.00 COMMENT '运费',
    
    PRIMARY KEY (order_id),
    INDEX idx_user_id (user_id),
    INDEX idx_status (status),
    
    FOREIGN KEY (user_id) REFERENCES `User`(user_id)
    -- 注意：这里的 address_id 关联的是下单时的快照地址，通常不设ON DELETE/UPDATE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='订单表';

-- ----------------------------------------
-- 7. 订单详情表 (OrderDetail)
-- 记录订单中包含的商品及其当时的单价和数量
-- ----------------------------------------
CREATE TABLE `OrderDetail` (
    order_detail_id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '订单详情ID',
    order_id BIGINT UNSIGNED NOT NULL COMMENT '所属订单ID',
    product_id BIGINT UNSIGNED NOT NULL COMMENT '商品ID快照',
    product_name_snapshot VARCHAR(255) NOT NULL COMMENT '商品名称快照',
    quantity INT UNSIGNED NOT NULL COMMENT '购买数量',
    unit_price DECIMAL(10, 2) NOT NULL COMMENT '购买时的单价',
    
    PRIMARY KEY (order_detail_id),
    INDEX idx_order_id (order_id),
    
    -- 外键关联到 Order 表，级联删除 (CASCADE) 确保删除订单时，详情也删除
    FOREIGN KEY (order_id) REFERENCES `Order`(order_id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='订单详情表';

-- ----------------------------------------
-- 8. 购物车表 (Cart)
-- 记录用户购物车中的商品
-- ----------------------------------------
CREATE TABLE `Cart` (
    cart_id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '购物车ID',
    user_id BIGINT UNSIGNED NOT NULL COMMENT '用户ID',
    product_id BIGINT UNSIGNED NOT NULL COMMENT '商品ID',
    quantity INT UNSIGNED NOT NULL COMMENT '购买数量',
    
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后更新时间',
    
    PRIMARY KEY (cart_id),
    -- 保证一个用户的同一商品只有一条购物车记录
    UNIQUE KEY uk_user_product (user_id, product_id), 
    
    -- 外键
    FOREIGN KEY (user_id) REFERENCES `User`(user_id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES `Product`(product_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='购物车表';

-- ----------------------------------------
-- 9. 评论表 (Review) - 扩展功能
-- 记录用户对商品的评价
-- ----------------------------------------
CREATE TABLE `Review` (
    review_id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '评论ID',
    user_id BIGINT UNSIGNED NOT NULL COMMENT '用户ID',
    product_id BIGINT UNSIGNED NOT NULL COMMENT '商品ID',
    rating TINYINT UNSIGNED NOT NULL COMMENT '评分 (1-5星)',
    content TEXT NULL COMMENT '评论内容',
    
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '评论时间',
    
    PRIMARY KEY (review_id),
    INDEX idx_product_id (product_id),
    
    FOREIGN KEY (user_id) REFERENCES `User`(user_id),
    FOREIGN KEY (product_id) REFERENCES `Product`(product_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品评论表';